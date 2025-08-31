//! Registration interface and logic for CRIMSON-REDLINE

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

/// Registration screen state
pub struct RegisterScreen {
    username: String,
    password: String,
    confirm_password: String,
    show_password: bool,
    input_mode: InputMode,
    error_message: Option<String>,
    success_message: Option<String>,
    password_strength: PasswordStrength,
}

#[derive(Debug, PartialEq)]
enum InputMode {
    Username,
    Password,
    ConfirmPassword,
}

#[derive(Debug, PartialEq)]
enum PasswordStrength {
    None,
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

impl RegisterScreen {
    /// Create a new registration screen
    pub fn new() -> Self {
        RegisterScreen {
            username: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            show_password: false,
            input_mode: InputMode::Username,
            error_message: None,
            success_message: None,
            password_strength: PasswordStrength::None,
        }
    }

    /// Calculate password strength
    fn calculate_password_strength(password: &str) -> PasswordStrength {
        if password.is_empty() {
            return PasswordStrength::None;
        }

        let mut score = 0;
        
        // Length scoring
        if password.len() >= 8 { score += 1; }
        if password.len() >= 12 { score += 1; }
        if password.len() >= 16 { score += 1; }
        
        // Character variety scoring
        if password.chars().any(|c| c.is_lowercase()) { score += 1; }
        if password.chars().any(|c| c.is_uppercase()) { score += 1; }
        if password.chars().any(|c| c.is_ascii_digit()) { score += 1; }
        if password.chars().any(|c| !c.is_alphanumeric()) { score += 1; }
        
        match score {
            0..=2 => PasswordStrength::Weak,
            3..=4 => PasswordStrength::Medium,
            5..=6 => PasswordStrength::Strong,
            _ => PasswordStrength::VeryStrong,
        }
    }

    /// Get password strength display
    fn get_strength_display(&self) -> &str {
        match self.password_strength {
            PasswordStrength::None => "",
            PasswordStrength::Weak => "[WEAK - EASILY COMPROMISED]",
            PasswordStrength::Medium => "[MEDIUM - MODERATE SECURITY]",
            PasswordStrength::Strong => "[STRONG - GOOD SECURITY]",
            PasswordStrength::VeryStrong => "[VERY STRONG - EXCELLENT SECURITY]",
        }
    }

    /// Display the registration screen
    pub async fn display(&self, color_scheme: &ColorScheme) -> Result<()> {
        execute!(
            io::stdout(),
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        // Display header
        color_scheme.print_colored("\n")?;
        color_scheme.print_colored("    ╔═══════════════════════════════════════════════════════════╗\n")?;
        color_scheme.print_colored("    ║                                                           ║\n")?;
        color_scheme.print_colored("    ║          C R E A T E   N E W   A G E N T                 ║\n")?;
        color_scheme.print_colored("    ║                                                           ║\n")?;
        color_scheme.print_colored("    ╚═══════════════════════════════════════════════════════════╝\n")?;
        color_scheme.print_colored("\n")?;

        // Show messages
        if let Some(ref error) = self.error_message {
            color_scheme.print_error(&format!("    [!] {}\n", error))?;
            color_scheme.print_colored("\n")?;
        }
        
        if let Some(ref success) = self.success_message {
            color_scheme.print_success(&format!("    [✓] {}\n", success))?;
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
        color_scheme.print_colored("     (3-20 characters, letters, numbers, underscore only)\n")?;
        color_scheme.print_colored("\n")?;

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
        
        // Show password strength
        if !self.password.is_empty() {
            let strength_display = self.get_strength_display();
            match self.password_strength {
                PasswordStrength::Weak => color_scheme.print_error(&format!("     {}\n", strength_display))?,
                PasswordStrength::Medium => color_scheme.print_warning(&format!("     {}\n", strength_display))?,
                PasswordStrength::Strong | PasswordStrength::VeryStrong => 
                    color_scheme.print_success(&format!("     {}\n", strength_display))?,
                _ => {}
            }
        }
        color_scheme.print_colored("     (8+ chars, uppercase, lowercase, number, special char)\n")?;
        color_scheme.print_colored("\n")?;

        // Confirm password field
        let confirm_prefix = if self.input_mode == InputMode::ConfirmPassword { " >" } else { "  " };
        let confirm_display = if self.show_password {
            self.confirm_password.clone()
        } else {
            "*".repeat(self.confirm_password.len())
        };
        
        color_scheme.print_colored(&format!(
            "   {}CONFIRM PASSWORD: {}\n",
            confirm_prefix,
            if self.input_mode == InputMode::ConfirmPassword {
                format!("{}█", confirm_display)
            } else {
                confirm_display
            }
        ))?;
        
        // Show password match status
        if !self.confirm_password.is_empty() {
            if self.password == self.confirm_password {
                color_scheme.print_success("     [PASSWORDS MATCH]\n")?;
            } else {
                color_scheme.print_error("     [PASSWORDS DO NOT MATCH]\n")?;
            }
        }

        color_scheme.print_colored("\n")?;
        color_scheme.print_colored("    ───────────────────────────────────────────────────────────\n")?;
        color_scheme.print_colored("    [TAB] Next Field  [SHIFT+TAB] Previous  [ENTER] Create\n")?;
        color_scheme.print_colored("    [F1] Toggle Password Visibility  [ESC] Cancel\n")?;

        io::stdout().flush()?;
        Ok(())
    }

    /// Handle keyboard input
    pub fn handle_input(&mut self, key: KeyEvent) -> RegisterAction {
        match key.code {
            KeyCode::Tab => {
                if key.modifiers.contains(event::KeyModifiers::SHIFT) {
                    // Previous field
                    self.input_mode = match self.input_mode {
                        InputMode::Username => InputMode::ConfirmPassword,
                        InputMode::Password => InputMode::Username,
                        InputMode::ConfirmPassword => InputMode::Password,
                    };
                } else {
                    // Next field
                    self.input_mode = match self.input_mode {
                        InputMode::Username => InputMode::Password,
                        InputMode::Password => InputMode::ConfirmPassword,
                        InputMode::ConfirmPassword => InputMode::Username,
                    };
                }
                self.error_message = None;
                RegisterAction::Continue
            }
            KeyCode::Enter => {
                self.validate_and_register()
            }
            KeyCode::Esc => RegisterAction::Cancel,
            KeyCode::F(1) => {
                self.show_password = !self.show_password;
                RegisterAction::Continue
            }
            KeyCode::Backspace => {
                match self.input_mode {
                    InputMode::Username => {
                        self.username.pop();
                    }
                    InputMode::Password => {
                        self.password.pop();
                        self.password_strength = Self::calculate_password_strength(&self.password);
                    }
                    InputMode::ConfirmPassword => {
                        self.confirm_password.pop();
                    }
                }
                self.error_message = None;
                RegisterAction::Continue
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
                            self.password_strength = Self::calculate_password_strength(&self.password);
                        }
                    }
                    InputMode::ConfirmPassword => {
                        if self.confirm_password.len() < 50 {
                            self.confirm_password.push(c);
                        }
                    }
                }
                self.error_message = None;
                RegisterAction::Continue
            }
            _ => RegisterAction::Continue,
        }
    }

    /// Validate inputs and attempt registration
    fn validate_and_register(&mut self) -> RegisterAction {
        // Clear previous messages
        self.error_message = None;
        self.success_message = None;

        // Validate all fields are filled
        if self.username.is_empty() {
            self.error_message = Some("Username is required".to_string());
            return RegisterAction::Continue;
        }
        
        if self.password.is_empty() {
            self.error_message = Some("Password is required".to_string());
            return RegisterAction::Continue;
        }
        
        if self.confirm_password.is_empty() {
            self.error_message = Some("Please confirm your password".to_string());
            return RegisterAction::Continue;
        }

        // Validate passwords match
        if self.password != self.confirm_password {
            self.error_message = Some("Passwords do not match".to_string());
            return RegisterAction::Continue;
        }

        // All validations passed
        RegisterAction::AttemptRegister
    }

    /// Process registration attempt
    pub async fn attempt_register(&mut self, auth: &mut AuthSystem) -> Result<bool> {
        // Show loading animation
        animations::show_processing("CREATING AGENT PROFILE", 2000).await?;
        
        match auth.register(&self.username, &self.password, &self.confirm_password).await {
            Ok(user) => {
                // Clear sensitive data
                self.password.clear();
                self.confirm_password.clear();
                
                // Show success message
                animations::show_success(&format!(
                    "AGENT {} SUCCESSFULLY CREATED - PROCEED TO LOGIN",
                    user.username
                )).await?;
                
                self.success_message = Some(format!("Agent {} created successfully!", user.username));
                Ok(true)
            }
            Err(e) => {
                self.error_message = Some(e.to_string());
                Ok(false)
            }
        }
    }

    /// Clear all fields
    pub fn clear(&mut self) {
        self.username.clear();
        self.password.clear();
        self.confirm_password.clear();
        self.show_password = false;
        self.input_mode = InputMode::Username;
        self.error_message = None;
        self.success_message = None;
        self.password_strength = PasswordStrength::None;
    }
}

/// Registration action enum
#[derive(Debug, PartialEq)]
pub enum RegisterAction {
    Continue,
    AttemptRegister,
    Cancel,
}

/// Run the registration interface
pub async fn run_registration(auth: &mut AuthSystem, color_scheme: &ColorScheme) -> Result<bool> {
    let mut register_screen = RegisterScreen::new();
    
    crossterm::terminal::enable_raw_mode()?;
    
    loop {
        register_screen.display(color_scheme).await?;
        
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match register_screen.handle_input(key) {
                    RegisterAction::Continue => continue,
                    RegisterAction::AttemptRegister => {
                        let success = register_screen.attempt_register(auth).await?;
                        if success {
                            // Wait for user to see success message
                            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                            crossterm::terminal::disable_raw_mode()?;
                            return Ok(true);
                        }
                    }
                    RegisterAction::Cancel => {
                        crossterm::terminal::disable_raw_mode()?;
                        return Ok(false);
                    }
                }
            }
        }
    }
}