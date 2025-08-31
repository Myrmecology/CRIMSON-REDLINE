//! Menu system for CRIMSON-REDLINE

use crate::ui::{ColorScheme, ascii_art};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{Clear, ClearType},
    cursor,
    execute,
};
use std::io::{self, Write};
use anyhow::Result;

/// Main menu structure
pub struct MainMenu {
    options: Vec<MenuOption>,
    selected_index: usize,
    title: String,
    show_skull: bool,
}

/// Menu option
#[derive(Debug, Clone)]
pub struct MenuOption {
    pub label: String,
    pub action: MenuAction,
    pub description: String,
}

/// Menu actions
#[derive(Debug, Clone, PartialEq)]
pub enum MenuAction {
    CreateNewUser,
    Login,
    Exit,
    Continue,
    Back,
    // Terminal commands
    Help,
    Scan,
    Exploit,
    Decrypt,
    Inject,
    Trace,
    Status,
    Clear,
    Logout,
    Mission,
    DarkWeb,
    Firewall,
}

impl MainMenu {
    /// Create the initial entry menu
    pub fn entry_menu() -> Self {
        MainMenu {
            options: vec![
                MenuOption {
                    label: "CREATE NEW USER".to_string(),
                    action: MenuAction::CreateNewUser,
                    description: "Register a new agent profile".to_string(),
                },
                MenuOption {
                    label: "LOGIN".to_string(),
                    action: MenuAction::Login,
                    description: "Access existing agent profile".to_string(),
                },
                MenuOption {
                    label: "EXIT".to_string(),
                    action: MenuAction::Exit,
                    description: "Terminate connection".to_string(),
                },
            ],
            selected_index: 0,
            title: "SYSTEM ACCESS".to_string(),
            show_skull: true,
        }
    }

    /// Create the main terminal menu (after login)
    pub fn terminal_menu() -> Self {
        MainMenu {
            options: vec![
                MenuOption {
                    label: "help".to_string(),
                    action: MenuAction::Help,
                    description: "Display available commands".to_string(),
                },
                MenuOption {
                    label: "scan".to_string(),
                    action: MenuAction::Scan,
                    description: "Scan network for targets".to_string(),
                },
                MenuOption {
                    label: "exploit".to_string(),
                    action: MenuAction::Exploit,
                    description: "Deploy exploit on target".to_string(),
                },
                MenuOption {
                    label: "decrypt".to_string(),
                    action: MenuAction::Decrypt,
                    description: "Decrypt intercepted data".to_string(),
                },
                MenuOption {
                    label: "inject".to_string(),
                    action: MenuAction::Inject,
                    description: "Inject payload into system".to_string(),
                },
                MenuOption {
                    label: "trace".to_string(),
                    action: MenuAction::Trace,
                    description: "Trace connection route".to_string(),
                },
                MenuOption {
                    label: "status".to_string(),
                    action: MenuAction::Status,
                    description: "View agent status".to_string(),
                },
                MenuOption {
                    label: "mission".to_string(),
                    action: MenuAction::Mission,
                    description: "Access mission briefings".to_string(),
                },
                MenuOption {
                    label: "darkweb".to_string(),
                    action: MenuAction::DarkWeb,
                    description: "Access dark web marketplace".to_string(),
                },
                MenuOption {
                    label: "firewall".to_string(),
                    action: MenuAction::Firewall,
                    description: "Breach firewall defenses".to_string(),
                },
                MenuOption {
                    label: "clear".to_string(),
                    action: MenuAction::Clear,
                    description: "Clear terminal screen".to_string(),
                },
                MenuOption {
                    label: "logout".to_string(),
                    action: MenuAction::Logout,
                    description: "Disconnect from system".to_string(),
                },
            ],
            selected_index: 0,
            title: "COMMAND TERMINAL".to_string(),
            show_skull: false,
        }
    }

    /// Display the menu
    pub async fn display(&self, color_scheme: &ColorScheme) -> Result<()> {
        execute!(
            io::stdout(),
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        // Show skull art if enabled
        if self.show_skull {
            let skull = ascii_art::SKULL_SMALL;
            for line in skull.lines() {
                color_scheme.print_colored(line)?;
                println!();
            }
            println!();
        }

        // Display title with border
        color_scheme.print_colored("╔════════════════════════════════════════════════════════════╗\n")?;
        color_scheme.print_colored("║                                                            ║\n")?;
        let title_line = format!("║{:^60}║", self.title);
        color_scheme.print_colored(&title_line)?;
        println!();
        color_scheme.print_colored("║                                                            ║\n")?;
        color_scheme.print_colored("╚════════════════════════════════════════════════════════════╝\n")?;
        println!();

        // Display options
        for (index, option) in self.options.iter().enumerate() {
            if index == self.selected_index {
                color_scheme.print_bright("  ▶ ")?;
                color_scheme.print_bright(&format!("[{}]", option.label))?;
                color_scheme.print_secondary(&format!(" - {}", option.description))?;
            } else {
                color_scheme.print_dim("    ")?;
                color_scheme.print_colored(&format!("[{}]", option.label))?;
                color_scheme.print_dim(&format!(" - {}", option.description))?;
            }
            println!();
        }

        println!();
        color_scheme.print_colored("────────────────────────────────────────────────────────────\n")?;
        color_scheme.print_colored("[↑/↓] Navigate  [ENTER] Select  [ESC] Back\n")?;

        io::stdout().flush()?;
        Ok(())
    }

    /// Handle keyboard input
    pub fn handle_input(&mut self, key: KeyEvent) -> MenuAction {
        match key.code {
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                } else {
                    self.selected_index = self.options.len() - 1;
                }
                MenuAction::Continue
            }
            KeyCode::Down => {
                self.selected_index = (self.selected_index + 1) % self.options.len();
                MenuAction::Continue
            }
            KeyCode::Enter => {
                self.options[self.selected_index].action.clone()
            }
            KeyCode::Esc => MenuAction::Back,
            _ => MenuAction::Continue,
        }
    }

    /// Get selected option
    pub fn get_selected(&self) -> &MenuOption {
        &self.options[self.selected_index]
    }

    /// Reset selection to first item
    pub fn reset_selection(&mut self) {
        self.selected_index = 0;
    }
}

/// Command prompt for terminal mode
pub struct CommandPrompt {
    command: String,
    history: Vec<String>,
    history_index: Option<usize>,
    username: String,
}

impl CommandPrompt {
    /// Create a new command prompt
    pub fn new(username: String) -> Self {
        CommandPrompt {
            command: String::new(),
            history: Vec::new(),
            history_index: None,
            username,
        }
    }

    /// Display the command prompt
    pub fn display(&self, color_scheme: &ColorScheme) -> Result<()> {
        color_scheme.print_colored(&format!("{}@crimson", self.username))?;
        color_scheme.print_bright(":~# ")?;
        color_scheme.print_colored(&self.command)?;
        color_scheme.print_colored("█")?;
        io::stdout().flush()?;
        Ok(())
    }

    /// Handle input for command prompt
    pub fn handle_input(&mut self, key: KeyEvent) -> Option<MenuAction> {
        match key.code {
            KeyCode::Enter => {
                if !self.command.is_empty() {
                    let cmd = self.command.trim().to_lowercase();
                    self.history.push(self.command.clone());
                    self.command.clear();
                    self.history_index = None;
                    
                    // Parse command to action
                    match cmd.as_str() {
                        "help" => Some(MenuAction::Help),
                        "scan" => Some(MenuAction::Scan),
                        "exploit" => Some(MenuAction::Exploit),
                        "decrypt" => Some(MenuAction::Decrypt),
                        "inject" => Some(MenuAction::Inject),
                        "trace" => Some(MenuAction::Trace),
                        "status" => Some(MenuAction::Status),
                        "mission" => Some(MenuAction::Mission),
                        "darkweb" => Some(MenuAction::DarkWeb),
                        "firewall" => Some(MenuAction::Firewall),
                        "clear" => Some(MenuAction::Clear),
                        "logout" | "exit" => Some(MenuAction::Logout),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            KeyCode::Backspace => {
                self.command.pop();
                None
            }
            KeyCode::Up => {
                if !self.history.is_empty() {
                    self.history_index = Some(match self.history_index {
                        None => self.history.len() - 1,
                        Some(0) => 0,
                        Some(i) => i - 1,
                    });
                    
                    if let Some(idx) = self.history_index {
                        self.command = self.history[idx].clone();
                    }
                }
                None
            }
            KeyCode::Down => {
                if let Some(idx) = self.history_index {
                    if idx < self.history.len() - 1 {
                        self.history_index = Some(idx + 1);
                        self.command = self.history[idx + 1].clone();
                    } else {
                        self.history_index = None;
                        self.command.clear();
                    }
                }
                None
            }
            KeyCode::Char(c) => {
                self.command.push(c);
                None
            }
            _ => None,
        }
    }

    /// Get command history
    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }

    /// Clear command
    pub fn clear_command(&mut self) {
        self.command.clear();
        self.history_index = None;
    }
}

/// Run menu interaction loop
pub async fn run_menu(menu: &mut MainMenu, color_scheme: &ColorScheme) -> Result<MenuAction> {
    crossterm::terminal::enable_raw_mode()?;
    
    loop {
        menu.display(color_scheme).await?;
        
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let action = menu.handle_input(key);
                if action != MenuAction::Continue {
                    crossterm::terminal::disable_raw_mode()?;
                    return Ok(action);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_creation() {
        let menu = MainMenu::entry_menu();
        assert_eq!(menu.options.len(), 3);
        assert_eq!(menu.selected_index, 0);
        assert_eq!(menu.title, "SYSTEM ACCESS");
    }

    #[test]
    fn test_command_prompt() {
        let mut prompt = CommandPrompt::new("testuser".to_string());
        
        // Test command input
        prompt.handle_input(KeyEvent::from(KeyCode::Char('h')));
        prompt.handle_input(KeyEvent::from(KeyCode::Char('e')));
        prompt.handle_input(KeyEvent::from(KeyCode::Char('l')));
        prompt.handle_input(KeyEvent::from(KeyCode::Char('p')));
        
        assert_eq!(prompt.command, "help");
        
        // Test enter returns action
        let action = prompt.handle_input(KeyEvent::from(KeyCode::Enter));
        assert_eq!(action, Some(MenuAction::Help));
        
        // Test command was added to history
        assert_eq!(prompt.history.len(), 1);
        assert_eq!(prompt.history[0], "help");
    }

    #[test]
    fn test_menu_navigation() {
        let mut menu = MainMenu::entry_menu();
        
        // Test down navigation
        menu.handle_input(KeyEvent::from(KeyCode::Down));
        assert_eq!(menu.selected_index, 1);
        
        // Test up navigation
        menu.handle_input(KeyEvent::from(KeyCode::Up));
        assert_eq!(menu.selected_index, 0);
        
        // Test wrap around
        menu.handle_input(KeyEvent::from(KeyCode::Up));
        assert_eq!(menu.selected_index, 2);
    }
}