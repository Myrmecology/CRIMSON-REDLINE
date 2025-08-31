//! Login interface and logic for CRIMSON-REDLINE

use crate::ui::{ColorScheme, animations};
use crate::auth::AuthSystem;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{Clear, ClearType},
    cursor,
    execute,
    style::{Color, Print, SetForegroundColor, ResetColor},
};
use std::io::{self, Write};
use anyhow::Result;

/// Login screen state
pub struct LoginScreen {
    username: String,
    password: String,
    show_password: bool,
    input_mode: InputMode,
    error_message: Option<String>,
    attempts: u32,
}

#[derive(Debug, PartialEq)]
enum InputMode {
    Username,
    Password,
}

impl LoginScreen {
    /// Create a new login screen
    pub fn new() -> Self {
        LoginScreen {
            username: String::new(),
            password: String::new(),
            show_password: false,
            input_mode: InputMode::Username,
            error_message: None,
            attempts: 0,
        }
    }

    /// Display the login screen
    pub async fn display(&self, color_scheme: &ColorScheme) -> Result<()> {
        execute!(
            io::stdout(),
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        // Display header with glitch effect
        color_scheme.print_colored("\n")?;
        color_scheme.print_colored("    ╔═══════════════════════════════════════════════════════════╗\n")?;
        color_scheme.print_colored("    ║                                                           ║\n")?;
        color_scheme.print_colored("    ║              S Y S T E M   A C C E S S                   ║\n")?;
        color_scheme.print_colored("    ║                                                           ║\n")?;
        color_scheme.print_colored("    ╚═══════════════════════════════════════════════════════════╝\n")?;
        color_scheme.print_colored("\n")?;

        // Show error message if any
        if let Some(ref error) = self.error_message {
            color_scheme.print_error(&format!("    [!] {}\n", error))?;
            color_scheme.print_colored("\n")?;
        }

        // Username field
        let username_prefix = if self.input_mode == InputMode::Username { " >" } else { "  " };
        color_scheme.print_colored(&format!(
            "   {}USERNAME: {}\n",
            username_prefix,
            if self.input_mode == InputMode::Username {
                format!("{}█", self.username)
            } else {
                self.username.clone()
            }
        ))?;

        // Password field
        let password_prefix = if self.input_mode == InputMode::Password { " >" } else { "  " };
        let password_display = if self.show_password {
            self.password.clone()
        } else {
            "*".repeat(self.password.len())
        };
        
        color_scheme.print_colored(&format!(
            "   {}PASSWORD: {}\n",
            password_prefix,
            if self.input_mode == InputMode::Password {
                format!("{}█", password_display)
            } else {
                password_display
            }
        ))?;

        color_scheme.print_colored("\n")?;
        color_scheme.print_colored("    ───────────────────────────────────────────────────────────\n")?;
        color_scheme.print_colored("    [TAB] Switch Field  [ENTER] Submit  [ESC] Back\n")?;
        
        if self.input_mode == InputMode::Password {
            color_scheme.print_colored("    [F1] Toggle Password Visibility\n")?;
        }

        // Show attempt counter if there have been failed attempts
        if self.attempts > 0 {
            color_scheme.print_colored(&format!(
                "\n    Login Attempts: {}/3\n",
                self.attempts
            ))?;
        }

        io::stdout().flush()?;
        Ok(())
    }

    /// Handle keyboard input
    pub fn handle_input(&mut self, key: KeyEvent) -> LoginAction {
        match key.code {
            KeyCode::Tab => {
                self.input_mode = match self.input_mode {
                    InputMode::Username => InputMode::Password,
                    InputMode::Password => InputMode::Username,
                };
                self.error_message = None;
                LoginAction::Continue
            }
            KeyCode::Enter => {
                if self.username.is_empty() || self.password.is_empty() {
                    self.error_message = Some("Username and password are required".to_string());
                    LoginAction::Continue
                } else {
                    LoginAction::AttemptLogin
                }
            }
            KeyCode::Esc => LoginAction::Cancel,
            KeyCode::F(1) if self.input_mode == InputMode::Password => {
                self.show_password = !self.show_password;
                LoginAction::Continue
            }
            KeyCode::Backspace => {
                match self.input_mode {
                    InputMode::Username => {
                        self.username.pop();
                    }
                    InputMode::Password => {
                        self.password.pop();
                    }
                }
                self.error_message = None;
                LoginAction::Continue
            }
            KeyCode::Char(c) => {
                match self.input_mode {
                    InputMode::Username => {
                        if self.username.len() < 20 && (c.is_alphanumeric() || c == '_') {
                            self.username.push(c);
                        }
                    }
                    InputMode::Password => {
                        if self.password.len() < 50 {
                            self.password.push(c);
                        }
                    }
                }
                self.error_message = None;
                LoginAction::Continue
            }
            _ => LoginAction::Continue,
        }
    }

    /// Process login attempt
    pub async fn attempt_login(&mut self, auth: &mut AuthSystem) -> Result<bool> {
        self.attempts += 1;
        
        // Show loading animation
        animations::show_processing("AUTHENTICATING", 1500).await?;
        
        match auth.login(&self.username, &self.password).await {
            Ok(user) => {
                // Clear sensitive data
                self.password.clear();
                
                // Show success message
                animations::show_success(&format!(
                    "ACCESS GRANTED - Welcome back, Agent {}",
                    user.username
                )).await?;
                
                Ok(true)
            }
            Err(e) => {
                self.error_message = Some(e.to_string());
                self.password.clear();
                
                if self.attempts >= 3 {
                    animations::show_error("MAXIMUM LOGIN ATTEMPTS EXCEEDED").await?;
                    return Ok(false);
                }
                
                Ok(false)
            }
        }
    }

    /// Get username and password
    pub fn get_credentials(&self) -> (&str, &str) {
        (&self.username, &self.password)
    }

    /// Clear all fields
    pub fn clear(&mut self) {
        self.username.clear();
        self.password.clear();
        self.show_password = false;
        self.input_mode = InputMode::Username;
        self.error_message = None;
        self.attempts = 0;
    }
}

/// Login action enum
#[derive(Debug, PartialEq)]
pub enum LoginAction {
    Continue,
    AttemptLogin,
    Cancel,
}

/// Run the login interface
pub async fn run_login(auth: &mut AuthSystem, color_scheme: &ColorScheme) -> Result<bool> {
    let mut login_screen = LoginScreen::new();
    
    crossterm::terminal::enable_raw_mode()?;
    
    loop {
        login_screen.display(color_scheme).await?;
        
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match login_screen.handle_input(key) {
                    LoginAction::Continue => continue,
                    LoginAction::AttemptLogin => {
                        let success = login_screen.attempt_login(auth).await?;
                        if success {
                            crossterm::terminal::disable_raw_mode()?;
                            return Ok(true);
                        }
                        if login_screen.attempts >= 3 {
                            crossterm::terminal::disable_raw_mode()?;
                            return Ok(false);
                        }
                    }
                    LoginAction::Cancel => {
                        crossterm::terminal::disable_raw_mode()?;
                        return Ok(false);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_screen_creation() {
        let screen = LoginScreen::new();
        assert_eq!(screen.username, "");
        assert_eq!(screen.password, "");
        assert_eq!(screen.input_mode, InputMode::Username);
        assert!(!screen.show_password);
    }

    #[test]
    fn test_input_handling() {
        let mut screen = LoginScreen::new();
        
        // Test character input
        screen.handle_input(KeyEvent::from(KeyCode::Char('t')));
        screen.handle_input(KeyEvent::from(KeyCode::Char('e')));
        screen.handle_input(KeyEvent::from(KeyCode::Char('s')));
        screen.handle_input(KeyEvent::from(KeyCode::Char('t')));
        assert_eq!(screen.username, "test");
        
        // Test tab switching
        let action = screen.handle_input(KeyEvent::from(KeyCode::Tab));
        assert_eq!(action, LoginAction::Continue);
        assert_eq!(screen.input_mode, InputMode::Password);
        
        // Test backspace
        screen.handle_input(KeyEvent::from(KeyCode::Char('p')));
        screen.handle_input(KeyEvent::from(KeyCode::Char('a')));
        screen.handle_input(KeyEvent::from(KeyCode::Backspace));
        assert_eq!(screen.password, "p");
    }
}