//! ASCII art assets for CRIMSON-REDLINE

/// Large skull ASCII art
pub const SKULL_LARGE: &str = r#"
        ███████████████████████████
      ██▀░░░░░░░░░░░░░░░░░░░░░▀██
    ██▀░░░░░░░░░░░░░░░░░░░░░░░░░▀██
   ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██
  ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██
 ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██
 ██░░▄▄▄▄▄▄▄░░░░░░░░░░░▄▄▄▄▄▄▄░░░░░██
 ██░░▀█████▀░░░░░░░░░░░▀█████▀░░░░░██
 ██░░░░▀█▀░░░░░░░░░░░░░░░▀█▀░░░░░░░██
 ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░██
  ██░░░░░░░▄▄▄▄▄▄▄▄▄▄▄▄░░░░░░░░░░██
   ██░░░░░░▀▀▀▀▀▀▀▀▀▀▀▀░░░░░░░░░██
    ██▄░░░░░░░░░░░░░░░░░░░░░░░▄██
      ██▄░░░░░░░░░░░░░░░░░░░▄██
        ███████████████████████
"#;

/// Small skull ASCII art
pub const SKULL_SMALL: &str = r#"
    ▄▄▄▄▄▄▄▄▄▄▄
   ███████████████
  █████████████████
  ███ ▀▀▀▀▀▀▀▀▀ ███
  ███ ▀▄   ▄▀   ███
  ████▄ ▀▀▀ ▄▄█████
   █████████████████
    ▀▀▀▀▀█████▀▀▀▀▀
         █████
"#;

/// System logo ASCII art
pub const SYSTEM_LOGO: &str = r#"
 ▄████▄   ██▀███   ██▓ ███▄ ▄███▓  ██████  ▒█████   ███▄    █ 
▒██▀ ▀█  ▓██ ▒ ██▒▓██▒▓██▒▀█▀ ██▒▒██    ▒ ▒██▒  ██▒ ██ ▀█   █ 
▒▓█    ▄ ▓██ ░▄█ ▒▒██▒▓██    ▓██░░ ▓██▄   ▒██░  ██▒▓██  ▀█ ██▒
▒▓▓▄ ▄██▒▒██▀▀█▄  ░██░▒██    ▒██   ▒   ██▒▒██   ██░▓██▒  ▐▌██▒
▒ ▓███▀ ░░██▓ ▒██▒░██░▒██▒   ░██▒▒██████▒▒░ ████▓▒░▒██░   ▓██░
░ ░▒ ▒  ░░ ▒▓ ░▒▓░░▓  ░ ▒░   ░  ░▒ ▒▓▒ ▒ ░░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
  ░  ▒     ░▒ ░ ▒░ ▒ ░░  ░      ░░ ░▒  ░ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░

           R E D L I N E   T E R M I N A L
"#;

/// Warning sign ASCII art
pub const WARNING_SIGN: &str = r#"
      ╱╲
     ╱  ╲
    ╱ ⚠  ╲
   ╱      ╲
  ╱________╲
"#;

/// Lock ASCII art
pub const LOCK: &str = r#"
    ┌───┐
    │   │
  ┌─┴───┴─┐
  │       │
  │  🔒   │
  │       │
  └───────┘
"#;

/// Access granted ASCII art
pub const ACCESS_GRANTED: &str = r#"
╔════════════════════════════════════════╗
║    A C C E S S   G R A N T E D         ║
╚════════════════════════════════════════╝
"#;

/// Access denied ASCII art
pub const ACCESS_DENIED: &str = r#"
╔════════════════════════════════════════╗
║     A C C E S S   D E N I E D          ║
╚════════════════════════════════════════╝
"#;

/// Terminal prompt ASCII art
pub const TERMINAL_PROMPT: &str = r#"
root@crimson:~# 
"#;

/// Network nodes ASCII art
pub const NETWORK_MAP: &str = r#"
         [SERVER-01]
              │
      ┌───────┼───────┐
      │       │       │
  [NODE-A] [NODE-B] [NODE-C]
      │       │       │
   [SUB-1] [SUB-2] [SUB-3]
"#;

/// Binary cascade
pub const BINARY_CASCADE: &str = r#"
01001000 01000001 01000011 01001011
00100000 01010100 01001000 01000101
00100000 01010000 01001100 01000001
01001110 01000101 01010100 00100001
"#;

/// Crosshair ASCII art
pub const CROSSHAIR: &str = r#"
      │
   ───┼───
      │
"#;

/// Biohazard symbol
pub const BIOHAZARD: &str = r#"
    ☣ ☣ ☣
   ☣     ☣
  ☣       ☣
   ☣     ☣
    ☣ ☣ ☣
"#;

/// Circuit pattern
pub const CIRCUIT_PATTERN: &str = r#"
─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─
 │ │ │ │ │ │ │ │ │ │ │ │ │ │ │
─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─
"#;

/// Virus symbol
pub const VIRUS: &str = r#"
    ╱◣ ◢╲
   ◢ ◈◈◈ ◣
   ◣ ◈◈◈ ◢
    ╲◢ ◣╱
"#;

/// Injection needle
pub const INJECTION: &str = r#"
═══════▶
"#;

/// Firewall ASCII art
pub const FIREWALL: &str = r#"
████████████████████████████████
██┏━━━━━━━━━━━━━━━━━━━━━━━━━━┓██
██┃  F I R E W A L L         ┃██
██┃  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ ┃██
██┗━━━━━━━━━━━━━━━━━━━━━━━━━━┛██
████████████████████████████████
"#;

/// Data stream
pub const DATA_STREAM: &str = r#"
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
"#;

/// Target acquired
pub const TARGET: &str = r#"
    ┌─────────────┐
    │  ╔═══════╗  │
    │  ║ TARGET ║  │
    │  ║   ◎    ║  │
    │  ╚═══════╝  │
    └─────────────┘
"#;

/// System boot sequence
pub const BOOT_SEQUENCE: &str = r#"
[■□□□□□□□□□] 10%  LOADING KERNEL...
[■■□□□□□□□□] 20%  INITIALIZING DRIVERS...
[■■■□□□□□□□] 30%  MOUNTING FILESYSTEMS...
[■■■■□□□□□□] 40%  STARTING SERVICES...
[■■■■■□□□□□] 50%  LOADING MODULES...
[■■■■■■□□□□] 60%  CONFIGURING NETWORK...
[■■■■■■■□□□] 70%  ESTABLISHING CONNECTIONS...
[■■■■■■■■□□] 80%  BYPASSING SECURITY...
[■■■■■■■■■□] 90%  ACCESSING MAINFRAME...
[■■■■■■■■■■] 100% SYSTEM READY
"#;

/// Get random glitch pattern
pub fn get_random_glitch() -> &'static str {
    use rand::Rng;
    let patterns = vec![
        "▓▓▒▒░░  ░░▒▒▓▓",
        "█▀▄▀█▀▄▀█▀▄▀█",
        "╬╬╬╬╬╬╬╬╬╬╬╬╬",
        "░▒▓█▓▒░▒▓█▓▒░",
        "▪▫▪▫▪▫▪▫▪▫▪▫▪",
        "┼┼┼┼┼┼┼┼┼┼┼┼┼",
        "◢◣◢◣◢◣◢◣◢◣◢◣◢",
        "▲▼▲▼▲▼▲▼▲▼▲▼▲",
    ];
    
    let mut rng = rand::thread_rng();
    patterns[rng.gen_range(0..patterns.len())]
}

/// Create a border box
pub fn create_box(width: usize, height: usize, title: Option<&str>) -> String {
    let mut result = String::new();
    
    // Top border
    result.push('╔');
    if let Some(t) = title {
        let title_str = format!("═[ {} ]═", t);
        result.push_str(&title_str);
        for _ in title_str.len()..width-2 {
            result.push('═');
        }
    } else {
        for _ in 0..width-2 {
            result.push('═');
        }
    }
    result.push('╗');
    result.push('\n');
    
    // Middle rows
    for _ in 0..height-2 {
        result.push('║');
        for _ in 0..width-2 {
            result.push(' ');
        }
        result.push('║');
        result.push('\n');
    }
    
    // Bottom border
    result.push('╚');
    for _ in 0..width-2 {
        result.push('═');
    }
    result.push('╝');
    
    result
}

/// Create a progress bar
pub fn create_progress_bar(progress: f32, width: usize) -> String {
    let filled = (width as f32 * progress) as usize;
    let empty = width - filled;
    
    let mut bar = String::from("[");
    for _ in 0..filled {
        bar.push('█');
    }
    for _ in 0..empty {
        bar.push('░');
    }
    bar.push(']');
    
    format!("{} {:.0}%", bar, progress * 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_box() {
        let box_str = create_box(10, 5, Some("TEST"));
        assert!(box_str.contains("TEST"));
        assert!(box_str.contains('╔'));
        assert!(box_str.contains('╝'));
    }

    #[test]
    fn test_progress_bar() {
        let bar = create_progress_bar(0.5, 10);
        assert!(bar.contains("50%"));
        assert!(bar.contains('█'));
        assert!(bar.contains('░'));
    }

    #[test]
    fn test_random_glitch() {
        let glitch = get_random_glitch();
        assert!(!glitch.is_empty());
    }
}