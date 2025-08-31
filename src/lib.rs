//! CRIMSON-REDLINE: Terminal Hacking Simulator
//! 
//! A sinister terminal-based hacking simulator with persistent user authentication,
//! featuring a complete red aesthetic, ASCII art, and interactive commands.

// Module declarations
pub mod auth;
pub mod ui;
pub mod commands;
pub mod game;
pub mod utils;

// Re-exports for convenient access
pub use auth::{AuthSystem, User};
pub use ui::{RedlineUI, ColorScheme};
pub use commands::CommandHandler;
pub use game::GameState;
pub use utils::Config;

// Project-wide constants
pub const APP_NAME: &str = "CRIMSON-REDLINE";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DATA_DIR_NAME: &str = ".crimson_redline";
pub const USER_DB_FILE: &str = "users.db";
pub const GAME_STATE_FILE: &str = "game_state.db";

// Result type alias for the entire application
pub type Result<T> = anyhow::Result<T>;

// ASCII art constants
pub const SKULL_ART: &str = r#"
     _____ _____ _____ _____ _____ _____ _____    _____ _____ ____  __    _____ _____ _____ 
    |     | __  |     |     |   __|     |   | |  | __  |   __|    \|  |  |     |   | |   __|
    |   --|    -|-   -| | | |__   |  |  | | | |  |    -|   __|  |  |  |__|-   -| | | |   __|
    |_____|__|__|_____|_|_|_|_____|_____|_|___|  |__|__|_____|____/|_____|_____|_|___|_____|
"#;

pub const WELCOME_ART: &str = r#"
    ╔═══════════════════════════════════════════════════════════════════════════╗
    ║                                                                           ║
    ║   ▄████▄   ██▀███   ██▓ ███▄ ▄███▓  ██████  ▒█████   ███▄    █          ║
    ║  ▒██▀ ▀█  ▓██ ▒ ██▒▓██▒▓██▒▀█▀ ██▒▒██    ▒ ▒██▒  ██▒ ██ ▀█   █          ║
    ║  ▒▓█    ▄ ▓██ ░▄█ ▒▒██▒▓██    ▓██░░ ▓██▄   ▒██░  ██▒▓██  ▀█ ██▒         ║
    ║  ▒▓▓▄ ▄██▒▒██▀▀█▄  ░██░▒██    ▒██   ▒   ██▒▒██   ██░▓██▒  ▐▌██▒         ║
    ║  ▒ ▓███▀ ░░██▓ ▒██▒░██░▒██▒   ░██▒▒██████▒▒░ ████▓▒░▒██░   ▓██░         ║
    ║  ░ ░▒ ▒  ░░ ▒▓ ░▒▓░░▓  ░ ▒░   ░  ░▒ ▒▓▒ ▒ ░░ ▒░▒░▒░ ░ ▒░   ▒ ▒          ║
    ║    ░  ▒     ░▒ ░ ▒░ ▒ ░░  ░      ░░ ░▒  ░ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░         ║
    ║                                                                           ║
    ║                    R E D L I N E   T E R M I N A L                       ║
    ║                                                                           ║
    ╚═══════════════════════════════════════════════════════════════════════════╝
"#;

// Error messages
pub const ERROR_INVALID_CREDENTIALS: &str = "ACCESS DENIED: Invalid credentials";
pub const ERROR_USER_EXISTS: &str = "ERROR: User already exists in system";
pub const ERROR_WEAK_PASSWORD: &str = "ERROR: Password does not meet security requirements";
pub const ERROR_SYSTEM_FAILURE: &str = "SYSTEM FAILURE: Unable to complete operation";

// Success messages  
pub const MSG_LOGIN_SUCCESS: &str = "ACCESS GRANTED: Authentication successful";
pub const MSG_USER_CREATED: &str = "SUCCESS: New agent profile created";
pub const MSG_LOGOUT: &str = "CONNECTION TERMINATED: Session ended";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(APP_NAME, "CRIMSON-REDLINE");
        assert!(!USER_DB_FILE.is_empty());
        assert!(!GAME_STATE_FILE.is_empty());
    }
}