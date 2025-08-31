//! Configuration management for CRIMSON-REDLINE

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub display: DisplayConfig,
    pub security: SecurityConfig,
    pub game: GameConfig,
}

/// Display-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub typing_speed_ms: u64,
    pub glitch_intensity: f32,
    pub use_animations: bool,
    pub color_theme: ColorTheme,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub min_password_length: usize,
    pub require_special_chars: bool,
    pub max_login_attempts: u32,
    pub session_timeout_minutes: u32,
    pub bcrypt_cost: u32,
}

/// Game configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub starting_reputation: i32,
    pub max_heat_level: u32,
    pub heat_decay_rate: f32,
    pub enable_random_events: bool,
    pub difficulty: Difficulty,
}

/// Color themes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorTheme {
    Crimson,      // Default red theme
    Blood,        // Darker red
    Neon,         // Bright red/pink
    Terminal,     // Classic green (easter egg)
}

/// Game difficulty levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Difficulty {
    Script,       // Easy mode
    Hacker,       // Normal mode
    Ghost,        // Hard mode
    Phantom,      // Extreme mode
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: DisplayConfig {
                typing_speed_ms: 15,
                glitch_intensity: 0.1,
                use_animations: true,
                color_theme: ColorTheme::Crimson,
            },
            security: SecurityConfig {
                min_password_length: 8,
                require_special_chars: true,
                max_login_attempts: 3,
                session_timeout_minutes: 30,
                bcrypt_cost: 12,
            },
            game: GameConfig {
                starting_reputation: 0,
                max_heat_level: 100,
                heat_decay_rate: 0.95,
                enable_random_events: true,
                difficulty: Difficulty::Hacker,
            },
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let contents = std::fs::read_to_string(&config_path)?;
            let config: Config = serde_json::from_str(&contents)?;
            Ok(config)
        } else {
            // Create default config
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, contents)?;
        Ok(())
    }

    /// Get the configuration file path
    fn config_path() -> Result<PathBuf> {
        let data_dir = crate::utils::get_data_dir()?;
        Ok(data_dir.join("config.json"))
    }

    /// Validate password against security requirements
    pub fn validate_password(&self, password: &str) -> Result<()> {
        if password.len() < self.security.min_password_length {
            anyhow::bail!(
                "Password must be at least {} characters long",
                self.security.min_password_length
            );
        }

        if self.security.require_special_chars {
            let has_special = password.chars().any(|c| {
                !c.is_alphanumeric() && !c.is_whitespace()
            });
            
            if !has_special {
                anyhow::bail!("Password must contain at least one special character");
            }
        }

        // Check for at least one uppercase letter
        if !password.chars().any(|c| c.is_uppercase()) {
            anyhow::bail!("Password must contain at least one uppercase letter");
        }

        // Check for at least one lowercase letter
        if !password.chars().any(|c| c.is_lowercase()) {
            anyhow::bail!("Password must contain at least one lowercase letter");
        }

        // Check for at least one digit
        if !password.chars().any(|c| c.is_ascii_digit()) {
            anyhow::bail!("Password must contain at least one number");
        }

        Ok(())
    }

    /// Get color values based on theme
    pub fn get_color_rgb(&self) -> (u8, u8, u8) {
        match self.display.color_theme {
            ColorTheme::Crimson => (220, 20, 60),    // Crimson red
            ColorTheme::Blood => (136, 8, 8),        // Dark blood red
            ColorTheme::Neon => (255, 16, 70),       // Neon red/pink
            ColorTheme::Terminal => (0, 255, 0),     // Classic terminal green
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.security.min_password_length, 8);
        assert!(config.security.require_special_chars);
    }

    #[test]
    fn test_password_validation() {
        let config = Config::default();
        
        // Too short
        assert!(config.validate_password("Pass1!").is_err());
        
        // No special char
        assert!(config.validate_password("Password123").is_err());
        
        // No uppercase
        assert!(config.validate_password("password123!").is_err());
        
        // No lowercase
        assert!(config.validate_password("PASSWORD123!").is_err());
        
        // No digit
        assert!(config.validate_password("Password!").is_err());
        
        // Valid password
        assert!(config.validate_password("Password123!").is_ok());
    }

    #[test]
    fn test_color_themes() {
        let mut config = Config::default();
        
        config.display.color_theme = ColorTheme::Crimson;
        assert_eq!(config.get_color_rgb(), (220, 20, 60));
        
        config.display.color_theme = ColorTheme::Blood;
        assert_eq!(config.get_color_rgb(), (136, 8, 8));
    }
}