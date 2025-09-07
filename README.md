## WELCOME TO CRIMSON REDLINE ##

For a video demo of this project please visit: https://www.youtube.com/watch?v=jHEcApXRvnQ 

# CRIMSON-REDLINE

A sinister terminal-based hacking simulator built with 100% Rust, featuring persistent user authentication, immersive red-themed aesthetics, and interactive cybersecurity gameplay.

## Overview

CRIMSON-REDLINE is a terminal hacking simulator that provides an authentic command-line interface experience with a complete red visual theme. Every element - from text to graphics to borders - is rendered in various shades of red, creating an immersive underground hacker atmosphere.

## Features

### Core Systems
- **Persistent User Authentication**: Secure user registration and login system with bcrypt password hashing
- **User Data Persistence**: All created users are saved and remembered between sessions
- **Mandatory Login**: Access to the terminal requires authentication
- **Session Management**: Track login counts, last login time, and user statistics

### Visual Design
- **100% Red Theme**: Complete monochrome red aesthetic throughout the application
- **ASCII Art**: Custom skulls, system logos, and visual elements
- **Glitch Effects**: Random text corruption, flickering, and matrix-style animations
- **Typed Text Effects**: Dramatic character-by-character text rendering
- **Loading Animations**: Progress bars, spinners, and processing indicators

### Hacking Commands
- `help` - Display available commands and usage
- `scan` - Network scanning with vulnerability detection
- `exploit` - Deploy exploits against identified vulnerabilities
- `decrypt` - Decrypt intercepted data and files
- `inject` - Inject payloads into target systems
- `trace` - Trace network routes to targets
- `status` - View agent status and statistics
- `mission` - Access mission briefings and objectives
- `darkweb` - Browse underground marketplace
- `firewall` - Analyze and breach firewall defenses
- `clear` - Clear terminal screen
- `logout` - Disconnect from system

### Game Mechanics
- **Reputation System**: Build your reputation as an elite hacker
- **Heat Level**: Manage detection risk (0-100%)
- **Credits**: In-game currency for tools and upgrades
- **Missions**: Complete objectives for rewards
- **Achievements**: Unlock accomplishments
- **Random Events**: Dynamic events that require quick decisions
- **Streak Bonuses**: Chain successful operations for multipliers
- **Level Progression**: Advance from Nobody to Mythical Hacker

### Security Features
- **Password Requirements**: Minimum 8 characters, uppercase, lowercase, number, special character
- **Account Lockout**: Automatic lockout after 5 failed login attempts
- **Secure Storage**: User data encrypted with bincode serialization
- **Session Timeout**: Configurable session management
- **Government-Grade .gitignore**: Maximum security configuration included

## Installation

### Prerequisites
- Rust 1.75 or higher
- Windows Terminal, VS Code terminal, or any terminal with ANSI color support
- Git

### Setup Instructions

1. Clone the repository:
```bash
git clone https://github.com/Myrmecology/CRIMSON-REDLINE.git
cd CRIMSON-REDLINE

Build the project:
cargo build --release

Run the application:
cargo run --release

Usage Guide
First Time Setup

Launch the application
Select "CREATE NEW USER" from the main menu
Enter a username (3-20 characters, alphanumeric and underscore only)
Create a strong password following the security requirements
Confirm your password
Return to main menu and LOGIN with your credentials

Terminal Commands
Once logged in, you'll enter the hacker terminal. Type commands directly at the prompt:

agent@crimson:~# scan
agent@crimson:~# exploit 192.168.1.1
agent@crimson:~# decrypt 0xABCDEF

Command Examples
Network Scanning
bashscan                    # Scan local network
scan 192.168.1.1       # Scan specific target
scan network -v        # Verbose network scan
Exploitation
bashexploit target_ip              # Auto-select exploit
exploit target_ip eternalblue  # Use specific exploit
exploit target_ip ms17-010     # Use by CVE number
Data Operations
bashdecrypt                 # Decrypt random data
decrypt encrypted_file  # Decrypt specific file
inject target trojan    # Inject trojan payload
Heat Management
Your heat level increases with each action:

Scanning: +10% heat
Exploiting: +25% heat
Injecting: +20% heat
Failed operations: +15% heat

Heat naturally decays over time. Keep it below 75% to avoid detection!
Reputation Levels
Progress through 11 reputation levels:

Nobody (0-49)
Wannabe (50-149)
Script Kiddie (150-299)
Amateur (300-499)
Competent (500-749)
Skilled (750-999)
Expert (1000-1499)
Master (1500-1999)
Elite (2000-2999)
Legendary (3000-4999)
Mythical (5000+)

Game Mechanics
Missions
Complete missions to earn reputation and credits:

INIT-001: First Steps (Trivial)
RECON-001: Network Reconnaissance (Easy)
DATA-001: Data Extraction (Medium)
CORP-001: Corporate Espionage (Hard)
GHOST-001: Ghost Protocol (Extreme)
IMPOSSIBLE-001: The Impossible (Impossible)

Random Events
Dynamic events can occur during operations:

Opportunities: Vulnerable systems, data caches, backdoors
Threats: Honeypots, rival hackers, AI defense systems
Critical Events: System lockdowns, trace initiations

Achievements
Unlock achievements for various accomplishments:

Welcome to the Grid (First login)
Script Kiddie (First exploit)
Codebreaker (Decrypt 50 files)
Ghost in the Machine (Stealth operations)
Master of the Digital Domain (Complete all content)

Technical Architecture
Project Structure
CRIMSON-REDLINE/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Module declarations
│   ├── auth/             # Authentication system
│   ├── ui/               # Terminal UI components
│   ├── commands/         # Hacking commands
│   ├── game/             # Game state management
│   └── utils/            # Utilities and config
├── Cargo.toml            # Dependencies
└── README.md             # Documentation

Dependencies

crossterm: Terminal manipulation and colors
ratatui: TUI framework
tokio: Async runtime for animations
bcrypt: Password hashing
serde: Data serialization
bincode: Binary encoding for storage
directories: OS-specific paths
chrono: Date and time handling
rand: Random number generation
anyhow/thiserror: Error handling

Data Storage
User data and game saves are stored in platform-specific locations:

Windows: %APPDATA%\crimson\redline\data\
Linux: ~/.local/share/crimson-redline/
macOS: ~/Library/Application Support/com.crimson.redline/

Performance

Compiled binary size: ~10-15 MB
Memory usage: < 50 MB
No internet connection required
Instant command response
Smooth 60 FPS animations

Development
Building from Source
bash# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with verbose output
RUST_LOG=debug cargo run

Keyboard Shortcuts
Login/Registration

TAB - Switch between fields
ENTER - Submit
ESC - Go back
F1 - Toggle password visibility

Main Menu

↑/↓ - Navigate options
ENTER - Select option
ESC - Back/Exit

Terminal

↑/↓ - Command history
CTRL+C - Cancel current operation
CTRL+L - Clear screen
ESC - Quick logout

Security Considerations

Passwords are hashed with bcrypt (cost factor 12)
User data is stored locally, never transmitted
Session data is cleared on logout
No telemetry or data collection
All operations are simulated, no real hacking

License
This project is licensed under the MIT License - see below for details

Please remember, this is not real it's a simulation, terminal video game, that's it, it's meant for fun. 

Acknowledgments

Built with Rust and love for the terminal aesthetic
Inspired by cyberpunk culture and hacker simulators
Thanks to the Rust community for excellent crates

Remember: This is a simulation. Always practice ethical hacking and respect computer security laws.

My inspiration for making this project was the movie Hackers. 