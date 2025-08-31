//! Color management and theming for CRIMSON-REDLINE

use crossterm::{
    style::{Color, SetForegroundColor, ResetColor, Attribute, SetAttribute},
    execute,
};
use std::io::{self, Write};
use anyhow::Result;
use crate::utils::config::ColorTheme;

/// Color scheme handler for the terminal
#[derive(Debug, Clone)]
pub struct ColorScheme {
    primary: Color,
    secondary: Color,
    error: Color,
    warning: Color,
    success: Color,
    dim: Color,
    bright: Color,
    theme: ColorTheme,
}

impl ColorScheme {
    /// Create default crimson color scheme
    pub fn new() -> Self {
        Self::from_theme(ColorTheme::Crimson)
    }

    /// Create color scheme from theme
    pub fn from_theme(theme: ColorTheme) -> Self {
        match theme {
            ColorTheme::Crimson => ColorScheme {
                primary: Color::Rgb { r: 220, g: 20, b: 60 },      // Crimson
                secondary: Color::Rgb { r: 178, g: 34, b: 34 },    // Firebrick
                error: Color::Rgb { r: 255, g: 0, b: 0 },          // Pure red
                warning: Color::Rgb { r: 255, g: 140, b: 0 },      // Dark orange
                success: Color::Rgb { r: 50, g: 205, b: 50 },      // Lime green
                dim: Color::Rgb { r: 139, g: 0, b: 0 },            // Dark red
                bright: Color::Rgb { r: 255, g: 69, b: 0 },        // Red-orange
                theme,
            },
            ColorTheme::Blood => ColorScheme {
                primary: Color::Rgb { r: 136, g: 8, b: 8 },        // Blood red
                secondary: Color::Rgb { r: 114, g: 0, b: 0 },      // Dark blood
                error: Color::Rgb { r: 204, g: 0, b: 0 },          // Bright blood
                warning: Color::Rgb { r: 204, g: 85, b: 0 },       // Rust
                success: Color::Rgb { r: 0, g: 100, b: 0 },        // Dark green
                dim: Color::Rgb { r: 64, g: 0, b: 0 },             // Very dark red
                bright: Color::Rgb { r: 170, g: 0, b: 0 },         // Medium blood
                theme,
            },
            ColorTheme::Neon => ColorScheme {
                primary: Color::Rgb { r: 255, g: 16, b: 70 },      // Neon red
                secondary: Color::Rgb { r: 255, g: 0, b: 128 },    // Hot pink
                error: Color::Rgb { r: 255, g: 0, b: 255 },        // Magenta
                warning: Color::Rgb { r: 255, g: 105, b: 180 },    // Hot pink
                success: Color::Rgb { r: 0, g: 255, b: 127 },      // Spring green
                dim: Color::Rgb { r: 199, g: 21, b: 133 },         // Medium violet
                bright: Color::Rgb { r: 255, g: 20, b: 147 },      // Deep pink
                theme,
            },
            ColorTheme::Terminal => ColorScheme {
                primary: Color::Rgb { r: 0, g: 255, b: 0 },        // Classic green
                secondary: Color::Rgb { r: 0, g: 200, b: 0 },      // Medium green
                error: Color::Rgb { r: 255, g: 0, b: 0 },          // Red
                warning: Color::Rgb { r: 255, g: 255, b: 0 },      // Yellow
                success: Color::Rgb { r: 0, g: 255, b: 255 },      // Cyan
                dim: Color::Rgb { r: 0, g: 128, b: 0 },            // Dark green
                bright: Color::Rgb { r: 127, g: 255, b: 0 },       // Chartreuse
                theme,
            },
        }
    }

    /// Print text in primary color
    pub fn print_colored(&self, text: &str) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(self.primary)
        )?;
        print!("{}", text);
        execute!(
            io::stdout(),
            ResetColor
        )?;
        Ok(())
    }

    /// Print text in secondary color
    pub fn print_secondary(&self, text: &str) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(self.secondary)
        )?;
        print!("{}", text);
        execute!(
            io::stdout(),
            ResetColor
        )?;
        Ok(())
    }

    /// Print error text
    pub fn print_error(&self, text: &str) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(self.error),
            SetAttribute(Attribute::Bold)
        )?;
        print!("{}", text);
        execute!(
            io::stdout(),
            SetAttribute(Attribute::Reset),
            ResetColor
        )?;
        Ok(())
    }

    /// Print warning text
    pub fn print_warning(&self, text: &str) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(self.warning)
        )?;
        print!("{}", text);
        execute!(
            io::stdout(),
            ResetColor
        )?;
        Ok(())
    }

    /// Print success text
    pub fn print_success(&self, text: &str) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(self.success),
            SetAttribute(Attribute::Bold)
        )?;
        print!("{}", text);
        execute!(
            io::stdout(),
            SetAttribute(Attribute::Reset),
            ResetColor
        )?;
        Ok(())
    }

    /// Print dim text
    pub fn print_dim(&self, text: &str) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(self.dim),
            SetAttribute(Attribute::Dim)
        )?;
        print!("{}", text);
        execute!(
            io::stdout(),
            SetAttribute(Attribute::Reset),
            ResetColor
        )?;
        Ok(())
    }

    /// Print bright/highlighted text
    pub fn print_bright(&self, text: &str) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(self.bright),
            SetAttribute(Attribute::Bold)
        )?;
        print!("{}", text);
        execute!(
            io::stdout(),
            SetAttribute(Attribute::Reset),
            ResetColor
        )?;
        Ok(())
    }

    /// Print text with blinking effect
    pub fn print_blinking(&self, text: &str) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(self.primary),
            SetAttribute(Attribute::SlowBlink)
        )?;
        print!("{}", text);
        execute!(
            io::stdout(),
            SetAttribute(Attribute::Reset),
            ResetColor
        )?;
        Ok(())
    }

    /// Print text with underline
    pub fn print_underlined(&self, text: &str) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(self.primary),
            SetAttribute(Attribute::Underlined)
        )?;
        print!("{}", text);
        execute!(
            io::stdout(),
            SetAttribute(Attribute::Reset),
            ResetColor
        )?;
        Ok(())
    }

    /// Set terminal to primary color (doesn't print)
    pub fn set_primary(&self) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(self.primary)
        )?;
        Ok(())
    }

    /// Reset terminal color
    pub fn reset(&self) -> Result<()> {
        execute!(
            io::stdout(),
            ResetColor
        )?;
        Ok(())
    }

    /// Get the primary color
    pub fn primary_color(&self) -> Color {
        self.primary
    }

    /// Get current theme
    pub fn theme(&self) -> &ColorTheme {
        &self.theme
    }

    /// Print with custom RGB color
    pub fn print_rgb(&self, text: &str, r: u8, g: u8, b: u8) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Rgb { r, g, b })
        )?;
        print!("{}", text);
        execute!(
            io::stdout(),
            ResetColor
        )?;
        Ok(())
    }

    /// Create a gradient effect between two colors
    pub fn print_gradient(&self, text: &str, from_rgb: (u8, u8, u8), to_rgb: (u8, u8, u8)) -> Result<()> {
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();
        
        if len == 0 {
            return Ok(());
        }

        for (i, ch) in chars.iter().enumerate() {
            let progress = i as f32 / (len - 1).max(1) as f32;
            
            let r = (from_rgb.0 as f32 + (to_rgb.0 as f32 - from_rgb.0 as f32) * progress) as u8;
            let g = (from_rgb.1 as f32 + (to_rgb.1 as f32 - from_rgb.1 as f32) * progress) as u8;
            let b = (from_rgb.2 as f32 + (to_rgb.2 as f32 - from_rgb.2 as f32) * progress) as u8;
            
            self.print_rgb(&ch.to_string(), r, g, b)?;
        }
        
        Ok(())
    }

    /// Print text with a glitch effect
    pub fn print_glitched(&self, text: &str, glitch_chance: f32) -> Result<()> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let glitch_chars = ['█', '▓', '▒', '░', '▀', '▄'];
        
        for ch in text.chars() {
            if rng.gen::<f32>() < glitch_chance {
                let glitch = glitch_chars[rng.gen_range(0..glitch_chars.len())];
                self.print_bright(&glitch.to_string())?;
            } else {
                self.print_colored(&ch.to_string())?;
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scheme_creation() {
        let scheme = ColorScheme::new();
        assert!(matches!(scheme.theme(), ColorTheme::Crimson));
    }

    #[test]
    fn test_theme_switching() {
        let blood_scheme = ColorScheme::from_theme(ColorTheme::Blood);
        assert!(matches!(blood_scheme.theme(), ColorTheme::Blood));
        
        let neon_scheme = ColorScheme::from_theme(ColorTheme::Neon);
        assert!(matches!(neon_scheme.theme(), ColorTheme::Neon));
    }
}