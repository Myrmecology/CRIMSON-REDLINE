//! User Interface modules for CRIMSON-REDLINE

pub mod animations;
pub mod ascii_art;
pub mod colors;
pub mod menu;

pub use colors::ColorScheme;
pub use menu::{MainMenu, MenuOption};
pub use animations::{show_intro, show_loading, show_processing};

use crossterm::{
    terminal::{Clear, ClearType, size},
    cursor,
    execute,
};
use std::io::{self, Write};
use anyhow::Result;

/// Main UI handler for CRIMSON-REDLINE
pub struct RedlineUI {
    color_scheme: ColorScheme,
    terminal_width: u16,
    terminal_height: u16,
}

impl RedlineUI {
    /// Create a new UI instance
    pub fn new() -> Result<Self> {
        let (width, height) = size()?;
        let color_scheme = ColorScheme::new();
        
        Ok(RedlineUI {
            color_scheme,
            terminal_width: width,
            terminal_height: height,
        })
    }

    /// Initialize the terminal UI
pub async fn initialize(&mut self) -> Result<()> {
    // Clear screen first
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::Hide
    )?;
    
    // Update terminal size
    let (width, height) = size()?;
    self.terminal_width = width;
    self.terminal_height = height;
    
    // Show intro animation
    animations::show_intro(&self.color_scheme).await?;
    
    // Clear screen after intro
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    
    Ok(())
}

    /// Cleanup terminal on exit
    pub fn cleanup(&self) -> Result<()> {
        execute!(
            io::stdout(),
            cursor::Show,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        Ok(())
    }

    /// Get color scheme reference
    pub fn color_scheme(&self) -> &ColorScheme {
        &self.color_scheme
    }

    /// Update color scheme
    pub fn set_color_scheme(&mut self, theme: crate::utils::config::ColorTheme) {
        self.color_scheme = ColorScheme::from_theme(theme);
    }

    /// Draw a bordered box
    pub fn draw_box(&self, x: u16, y: u16, width: u16, height: u16, title: Option<&str>) -> Result<()> {
        // Top border
        execute!(io::stdout(), cursor::MoveTo(x, y))?;
        self.color_scheme.print_colored("╔")?;
        
        if let Some(title) = title {
            let title_formatted = format!("═[ {} ]═", title);
            let padding = width.saturating_sub(title_formatted.len() as u16 + 2);
            self.color_scheme.print_colored(&title_formatted)?;
            for _ in 0..padding {
                self.color_scheme.print_colored("═")?;
            }
        } else {
            for _ in 0..width-2 {
                self.color_scheme.print_colored("═")?;
            }
        }
        self.color_scheme.print_colored("╗")?;

        // Side borders
        for i in 1..height-1 {
            execute!(io::stdout(), cursor::MoveTo(x, y + i))?;
            self.color_scheme.print_colored("║")?;
            execute!(io::stdout(), cursor::MoveTo(x + width - 1, y + i))?;
            self.color_scheme.print_colored("║")?;
        }

        // Bottom border
        execute!(io::stdout(), cursor::MoveTo(x, y + height - 1))?;
        self.color_scheme.print_colored("╚")?;
        for _ in 0..width-2 {
            self.color_scheme.print_colored("═")?;
        }
        self.color_scheme.print_colored("╝")?;

        Ok(())
    }

    /// Print centered text
    pub fn print_centered(&self, text: &str, y: u16) -> Result<()> {
        let x = self.terminal_width.saturating_sub(text.len() as u16) / 2;
        execute!(io::stdout(), cursor::MoveTo(x, y))?;
        self.color_scheme.print_colored(text)?;
        Ok(())
    }

    /// Show a notification message
    pub async fn show_notification(&self, message: &str, duration_ms: u64) -> Result<()> {
        let box_width = (message.len() + 6).min(self.terminal_width as usize - 4) as u16;
        let box_height = 5;
        let x = (self.terminal_width - box_width) / 2;
        let y = self.terminal_height / 2 - box_height / 2;

        // Draw notification box
        self.draw_box(x, y, box_width, box_height, Some("NOTIFICATION"))?;
        
        // Print message
        execute!(io::stdout(), cursor::MoveTo(x + 2, y + 2))?;
        self.color_scheme.print_colored(message)?;
        
        io::stdout().flush()?;
        
        // Wait
        tokio::time::sleep(tokio::time::Duration::from_millis(duration_ms)).await;
        
        Ok(())
    }

    /// Show a progress bar
    pub async fn show_progress(&self, title: &str, progress: f32) -> Result<()> {
        let bar_width = 40;
        let filled = (bar_width as f32 * progress) as usize;
        let empty = bar_width - filled;
        
        let y = self.terminal_height / 2;
        let x = (self.terminal_width - bar_width as u16 - 10) / 2;
        
        execute!(io::stdout(), cursor::MoveTo(x, y))?;
        self.color_scheme.print_colored(title)?;
        self.color_scheme.print_colored(" [")?;
        
        for _ in 0..filled {
            self.color_scheme.print_colored("█")?;
        }
        for _ in 0..empty {
            self.color_scheme.print_colored("░")?;
        }
        
        self.color_scheme.print_colored("] ")?;
        self.color_scheme.print_colored(&format!("{:.0}%", progress * 100.0))?;
        
        io::stdout().flush()?;
        Ok(())
    }

    /// Clear a specific area
    pub fn clear_area(&self, x: u16, y: u16, width: u16, height: u16) -> Result<()> {
        let blank = " ".repeat(width as usize);
        for i in 0..height {
            execute!(io::stdout(), cursor::MoveTo(x, y + i))?;
            print!("{}", blank);
        }
        io::stdout().flush()?;
        Ok(())
    }

    /// Draw a horizontal line
    pub fn draw_horizontal_line(&self, y: u16, style: LineStyle) -> Result<()> {
        execute!(io::stdout(), cursor::MoveTo(0, y))?;
        
        let line_char = match style {
            LineStyle::Single => "─",
            LineStyle::Double => "═",
            LineStyle::Dashed => "╌",
            LineStyle::Dotted => "┈",
        };
        
        for _ in 0..self.terminal_width {
            self.color_scheme.print_colored(line_char)?;
        }
        
        Ok(())
    }

    /// Get terminal dimensions
    pub fn dimensions(&self) -> (u16, u16) {
        (self.terminal_width, self.terminal_height)
    }

    /// Refresh terminal dimensions
    pub fn refresh_dimensions(&mut self) -> Result<()> {
        let (width, height) = size()?;
        self.terminal_width = width;
        self.terminal_height = height;
        Ok(())
    }
}

/// Line drawing styles
#[derive(Debug, Clone, Copy)]
pub enum LineStyle {
    Single,
    Double,
    Dashed,
    Dotted,
}

/// Create a glitch effect string
pub fn create_glitch(text: &str, intensity: f32) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let glitch_chars = ['█', '▓', '▒', '░', '▀', '▄', '■', '□', '▪', '▫'];
    
    text.chars()
        .map(|c| {
            if rng.gen::<f32>() < intensity {
                glitch_chars[rng.gen_range(0..glitch_chars.len())]
            } else {
                c
            }
        })
        .collect()
}

/// Type text effect character by character
pub async fn type_text(text: &str, delay_ms: u64, color_scheme: &ColorScheme) -> Result<()> {
    for ch in text.chars() {
        color_scheme.print_colored(&ch.to_string())?;
        io::stdout().flush()?;
        tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glitch_creation() {
        let original = "HELLO WORLD";
        let glitched = create_glitch(original, 0.5);
        assert_eq!(glitched.len(), original.len());
    }

    #[test]
    fn test_line_styles() {
        let styles = vec![
            LineStyle::Single,
            LineStyle::Double,
            LineStyle::Dashed,
            LineStyle::Dotted,
        ];
        
        for style in styles {
            match style {
                LineStyle::Single => assert_eq!(format!("{:?}", style), "Single"),
                LineStyle::Double => assert_eq!(format!("{:?}", style), "Double"),
                LineStyle::Dashed => assert_eq!(format!("{:?}", style), "Dashed"),
                LineStyle::Dotted => assert_eq!(format!("{:?}", style), "Dotted"),
            }
        }
    }
}