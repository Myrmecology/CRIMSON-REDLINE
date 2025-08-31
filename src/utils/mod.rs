//! Utility modules for CRIMSON-REDLINE

pub mod config;

pub use config::Config;

use std::path::PathBuf;
use directories::ProjectDirs;
use anyhow::Result;

/// Get the application's data directory
pub fn get_data_dir() -> Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "crimson", "redline") {
        let data_dir = proj_dirs.data_dir().to_path_buf();
        
        // Create directory if it doesn't exist
        if !data_dir.exists() {
            std::fs::create_dir_all(&data_dir)?;
        }
        
        Ok(data_dir)
    } else {
        // Fallback to current directory
        let fallback = PathBuf::from(".crimson_redline_data");
        if !fallback.exists() {
            std::fs::create_dir_all(&fallback)?;
        }
        Ok(fallback)
    }
}

/// Clear the terminal screen
pub fn clear_screen() -> Result<()> {
    use crossterm::{terminal::{Clear, ClearType}, execute};
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    Ok(())
}

/// Generate a random delay in milliseconds for dramatic effect
pub fn random_delay() -> u64 {
    use rand::Rng;
    rand::thread_rng().gen_range(50..200)
}

/// Sleep for a random dramatic delay
pub async fn dramatic_pause() {
    tokio::time::sleep(tokio::time::Duration::from_millis(random_delay())).await;
}

/// Create a glitched version of text
pub fn glitch_text(text: &str, intensity: f32) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let glitch_chars = vec!['█', '▓', '▒', '░', '╬', '╪', '┼', '╫'];
    
    text.chars()
        .map(|c| {
            if c.is_whitespace() {
                c
            } else if rng.gen::<f32>() < intensity {
                glitch_chars[rng.gen_range(0..glitch_chars.len())]
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glitch_text() {
        let original = "HELLO";
        let glitched = glitch_text(original, 0.0);
        assert_eq!(glitched, original);
        
        let fully_glitched = glitch_text(original, 1.0);
        assert_ne!(fully_glitched, original);
    }

    #[test]
    fn test_random_delay() {
        let delay = random_delay();
        assert!(delay >= 50 && delay < 200);
    }
}