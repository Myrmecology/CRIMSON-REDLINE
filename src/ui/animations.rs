//! Animation effects for CRIMSON-REDLINE

use crate::ui::{ColorScheme, ascii_art};
use crossterm::{
    terminal::{Clear, ClearType},
    cursor,
    execute,
};
use std::io::{self, Write};
use anyhow::Result;
use rand::Rng;

/// Show the intro animation
pub async fn show_intro(color_scheme: &ColorScheme) -> Result<()> {
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::Hide
    )?;

    // Matrix-style rain effect
    matrix_rain(color_scheme, 1000).await?;
    
    // Clear and show skull
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0, 5)
    )?;
    
    // Type out skull with glitch effect
    let skull = ascii_art::SKULL_LARGE;
    for line in skull.lines() {
        for ch in line.chars() {
            if ch != ' ' {
                color_scheme.print_glitched(&ch.to_string(), 0.1)?;
            } else {
                print!(" ");
            }
            io::stdout().flush()?;
            tokio::time::sleep(tokio::time::Duration::from_micros(500)).await;
        }
        println!();
    }
    
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // System initialization text
    execute!(io::stdout(), cursor::MoveTo(0, 15))?;
    type_text_effect("INITIALIZING CRIMSON-REDLINE SYSTEM...", 30, color_scheme).await?;
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    println!("\n");
    let init_messages = vec![
        "[OK] Loading encryption modules...",
        "[OK] Establishing secure channel...",
        "[OK] Bypassing security protocols...",
        "[OK] Accessing dark network...",
        "[OK] System ready.",
    ];
    
    for msg in init_messages {
        color_scheme.print_success("  ")?;
        type_text_effect(msg, 15, color_scheme).await?;
        println!();
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
    
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    execute!(io::stdout(), cursor::Show)?;
    Ok(())
}

/// Matrix rain effect
pub async fn matrix_rain(color_scheme: &ColorScheme, duration_ms: u64) -> Result<()> {
    let (width, height) = crossterm::terminal::size()?;
    let mut rng = rand::thread_rng();
    
    // Initialize columns with random positions
    let mut columns: Vec<i16> = (0..width)
        .map(|_| rng.gen_range(-(height as i16)..0))
        .collect();
    
    let start = tokio::time::Instant::now();
    let duration = tokio::time::Duration::from_millis(duration_ms);
    
    while start.elapsed() < duration {
        for (x, y) in columns.iter_mut().enumerate() {
            if *y >= 0 && *y < height as i16 {
                execute!(io::stdout(), cursor::MoveTo(x as u16, *y as u16))?;
                
                // Character selection
                let ch = if rng.gen::<f32>() < 0.7 {
                    rng.gen_range(0x30..0x39) as u8 as char  // Numbers
                } else {
                    rng.gen_range(0x41..0x5A) as u8 as char  // Letters
                };
                
                // Brightness based on position
                if *y == height as i16 - 1 {
                    color_scheme.print_bright(&ch.to_string())?;
                } else if *y > height as i16 - 5 {
                    color_scheme.print_colored(&ch.to_string())?;
                } else {
                    color_scheme.print_dim(&ch.to_string())?;
                }
            }
            
            *y += 1;
            
            // Reset column when it goes off screen
            if *y >= height as i16 + rng.gen_range(5..15) {
                *y = -(rng.gen_range(5..20));
            }
        }
        
        io::stdout().flush()?;
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    
    Ok(())
}

/// Show loading animation
pub async fn show_loading(message: &str, duration_ms: u64) -> Result<()> {
    let frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let color_scheme = ColorScheme::new();
    
    let start = tokio::time::Instant::now();
    let duration = tokio::time::Duration::from_millis(duration_ms);
    
    let mut frame_idx = 0;
    
    while start.elapsed() < duration {
        execute!(io::stdout(), cursor::SavePosition)?;
        color_scheme.print_colored(&format!("{} {}", frames[frame_idx], message))?;
        execute!(io::stdout(), cursor::RestorePosition)?;
        
        frame_idx = (frame_idx + 1) % frames.len();
        tokio::time::sleep(tokio::time::Duration::from_millis(80)).await;
    }
    
    // Clear the loading message
    execute!(io::stdout(), cursor::SavePosition)?;
    print!("{}", " ".repeat(message.len() + 3));
    execute!(io::stdout(), cursor::RestorePosition)?;
    
    Ok(())
}

/// Show processing animation with progress
pub async fn show_processing(task: &str, duration_ms: u64) -> Result<()> {
    let color_scheme = ColorScheme::new();
    let steps = 20;
    let step_duration = duration_ms / steps;
    
    println!();
    color_scheme.print_colored(&format!("  [*] {}...\n", task))?;
    print!("  [");
    
    for i in 0..steps {
        if i < steps / 3 {
            color_scheme.print_dim("█")?;
        } else if i < 2 * steps / 3 {
            color_scheme.print_colored("█")?;
        } else {
            color_scheme.print_bright("█")?;
        }
        io::stdout().flush()?;
        tokio::time::sleep(tokio::time::Duration::from_millis(step_duration)).await;
    }
    
    println!("]");
    Ok(())
}

/// Type text effect
pub async fn type_text_effect(text: &str, delay_ms: u64, color_scheme: &ColorScheme) -> Result<()> {
    for ch in text.chars() {
        color_scheme.print_colored(&ch.to_string())?;
        io::stdout().flush()?;
        tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
    }
    Ok(())
}

/// Glitch transition effect
pub async fn glitch_transition(color_scheme: &ColorScheme) -> Result<()> {
    let (width, height) = crossterm::terminal::size()?;
    let mut rng = rand::thread_rng();
    
    for _ in 0..10 {
        execute!(io::stdout(), Clear(ClearType::All))?;
        
        for _ in 0..rng.gen_range(50..200) {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            execute!(io::stdout(), cursor::MoveTo(x, y))?;
            
            let glitch_char = ['█', '▓', '▒', '░', '▀', '▄'][rng.gen_range(0..6)];
            color_scheme.print_glitched(&glitch_char.to_string(), 0.5)?;
        }
        
        io::stdout().flush()?;
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    
    execute!(io::stdout(), Clear(ClearType::All))?;
    Ok(())
}

/// Show success message with animation
pub async fn show_success(message: &str) -> Result<()> {
    let color_scheme = ColorScheme::new();
    
    println!();
    print!("  ");
    color_scheme.print_success("[✓] ")?;
    type_text_effect(message, 20, &color_scheme).await?;
    println!();
    
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    Ok(())
}

/// Show error message with animation
pub async fn show_error(message: &str) -> Result<()> {
    let color_scheme = ColorScheme::new();
    
    println!();
    print!("  ");
    color_scheme.print_error("[✗] ")?;
    
    // Glitch effect for error
    for ch in message.chars() {
        if rand::thread_rng().gen::<f32>() < 0.1 {
            color_scheme.print_glitched(&ch.to_string(), 0.5)?;
        } else {
            color_scheme.print_error(&ch.to_string())?;
        }
        io::stdout().flush()?;
        tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;
    }
    println!();
    
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    Ok(())
}

/// Scanning animation
pub async fn scanning_animation(target: &str, color_scheme: &ColorScheme) -> Result<()> {
    println!();
    color_scheme.print_colored(&format!("  [>] Scanning {}...\n", target))?;
    
    let scan_lines = vec![
        "  [>] Identifying open ports...",
        "  [>] Detecting vulnerabilities...",
        "  [>] Analyzing security protocols...",
        "  [>] Mapping network topology...",
        "  [>] Extracting system information...",
    ];
    
    for line in scan_lines {
        type_text_effect(line, 10, color_scheme).await?;
        println!();
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    }
    
    color_scheme.print_success("\n  [✓] Scan complete.\n")?;
    Ok(())
}

/// Decryption animation
pub async fn decryption_animation(data: &str, color_scheme: &ColorScheme) -> Result<()> {
    let mut rng = rand::thread_rng();
    let target = data;
    let mut current = String::new();
    
    // Fill with random characters initially
    for _ in target.chars() {
        current.push(rng.gen_range(33..126) as u8 as char);
    }
    
    println!();
    color_scheme.print_colored("  [>] Decrypting data...\n")?;
    print!("  ");
    
    // Gradually reveal the target text
    for i in 0..target.len() {
        for _ in 0..5 {
            execute!(io::stdout(), cursor::SavePosition)?;
            
            // Update random characters for undecrypted positions
            for (j, target_char) in target.chars().enumerate() {
                if j <= i {
                    color_scheme.print_success(&target_char.to_string())?;
                } else {
                    let random_char = rng.gen_range(33..126) as u8 as char;
                    color_scheme.print_dim(&random_char.to_string())?;
                }
            }
            
            execute!(io::stdout(), cursor::RestorePosition)?;
            io::stdout().flush()?;
            tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
        }
    }
    
    println!("\n");
    color_scheme.print_success("  [✓] Decryption complete.\n")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_type_text_effect() {
        let color_scheme = ColorScheme::new();
        // This should complete without error
        let result = type_text_effect("Test", 0, &color_scheme).await;
        assert!(result.is_ok());
    }
}