//! Command handler and executor for CRIMSON-REDLINE

use crate::commands::{CommandRegistry, scanner, exploit, decrypt};
use crate::game::GameState;
use crate::ui::{ColorScheme, animations};
use crate::auth::User;
use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;

/// Result of command execution
#[derive(Debug)]
pub enum CommandResult {
    Success(String),
    Error(String),
    Exit,
    Logout,
    Continue,
}

/// Main command handler
pub struct CommandHandler {
    registry: CommandRegistry,
    game_state: GameState,
    color_scheme: ColorScheme,
}

impl CommandHandler {
    /// Create a new command handler
    pub fn new(user: &User) -> Self {
        CommandHandler {
            registry: CommandRegistry::new(),
            game_state: GameState::new(user.username.clone(), user.reputation),
            color_scheme: ColorScheme::new(),
        }
    }

    /// Execute a command
    pub async fn execute(&mut self, input: &str) -> Result<CommandResult> {
        let input = input.trim();
        
        if input.is_empty() {
            return Ok(CommandResult::Continue);
        }

        let (command, args) = crate::commands::parse_args(input);
        
        match command.as_str() {
            "help" | "?" | "h" => self.handle_help(args).await,
            "scan" | "nmap" | "recon" => self.handle_scan(args).await,
            "exploit" | "pwn" | "attack" => self.handle_exploit(args).await,
            "decrypt" | "decode" | "decipher" => self.handle_decrypt(args).await,
            "inject" | "payload" | "implant" => self.handle_inject(args).await,
            "trace" | "traceroute" | "track" => self.handle_trace(args).await,
            "status" | "stats" | "info" => self.handle_status().await,
            "mission" | "objective" | "task" => self.handle_mission(args).await,
            "darkweb" | "market" | "underground" => self.handle_darkweb(args).await,
            "firewall" | "fw" | "barrier" => self.handle_firewall(args).await,
            "clear" | "cls" | "cl" => self.handle_clear().await,
            "logout" | "exit" | "quit" | "disconnect" => Ok(CommandResult::Logout),
            _ => {
                self.color_scheme.print_error(&format!("  [!] Unknown command: {}\n", command))?;
                self.color_scheme.print_dim("  Type 'help' for available commands\n")?;
                Ok(CommandResult::Continue)
            }
        }
    }

    /// Handle help command
    async fn handle_help(&self, args: Vec<String>) -> Result<CommandResult> {
        if args.is_empty() {
            self.registry.display_help(&self.color_scheme).await?;
        } else {
            self.registry.display_command_help(&args[0], &self.color_scheme).await?;
        }
        Ok(CommandResult::Continue)
    }

    /// Handle scan command
    async fn handle_scan(&mut self, args: Vec<String>) -> Result<CommandResult> {
        let target = if args.is_empty() {
            "network"
        } else {
            &args[0]
        };

        // Show scanning animation
        animations::scanning_animation(target, &self.color_scheme).await?;
        
        // Execute scan
        let results = scanner::execute_scan(target).await?;
        
        // Display results
        println!();
        self.color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        self.color_scheme.print_bright("                    SCAN RESULTS                               \n")?;
        self.color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        println!();
        
        for device in &results.devices {
            self.color_scheme.print_bright(&format!("  [+] {}\n", device.hostname))?;
            self.color_scheme.print_colored(&format!("      IP: {}\n", device.ip))?;
            self.color_scheme.print_colored(&format!("      MAC: {}\n", device.mac))?;
            self.color_scheme.print_colored(&format!("      OS: {}\n", device.os))?;
            
            if !device.open_ports.is_empty() {
                self.color_scheme.print_colored("      Open Ports: ")?;
                for port in &device.open_ports {
                    self.color_scheme.print_success(&format!("{} ", port))?;
                }
                println!();
            }
            
            if !device.vulnerabilities.is_empty() {
                self.color_scheme.print_warning("      Vulnerabilities:\n")?;
                for vuln in &device.vulnerabilities {
                    self.color_scheme.print_error(&format!("        - {}\n", vuln))?;
                }
            }
            println!();
        }
        
        self.color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        self.color_scheme.print_dim(&format!("  Total devices found: {}\n", results.devices.len()))?;
        self.color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        
        // Update game state
        self.game_state.add_reputation(5);
        self.game_state.increase_heat(10.0);
        
        Ok(CommandResult::Continue)
    }

    /// Handle exploit command
    async fn handle_exploit(&mut self, args: Vec<String>) -> Result<CommandResult> {
        if args.len() < 1 {
            self.color_scheme.print_error("  [!] Usage: exploit <target> [vulnerability_id]\n")?;
            return Ok(CommandResult::Continue);
        }

        let target = &args[0];
        let vuln_id = args.get(1).map(|s| s.as_str()).unwrap_or("auto");
        
        // Execute exploit
        let result = exploit::execute_exploit(target, vuln_id, &self.color_scheme).await?;
        
        if result.success {
            self.game_state.add_reputation(20);
            self.game_state.increase_heat(25.0);
            self.color_scheme.print_success(&format!("\n  [✓] Exploit successful! Gained {} reputation\n", 20))?;
        } else {
            self.game_state.increase_heat(15.0);
            self.color_scheme.print_error("\n  [✗] Exploit failed!\n")?;
        }
        
        Ok(CommandResult::Continue)
    }

    /// Handle decrypt command
    async fn handle_decrypt(&mut self, args: Vec<String>) -> Result<CommandResult> {
        if args.is_empty() {
            // Generate random encrypted data
            let encrypted = decrypt::generate_encrypted_data();
            animations::decryption_animation(&encrypted, &self.color_scheme).await?;
            
            let decrypted = decrypt::decrypt_data(&encrypted)?;
            self.color_scheme.print_success(&format!("\n  [✓] Decrypted: {}\n", decrypted))?;
        } else {
            let data = args.join(" ");
            animations::decryption_animation(&data, &self.color_scheme).await?;
            
            let decrypted = decrypt::decrypt_data(&data)?;
            self.color_scheme.print_success(&format!("\n  [✓] Decrypted: {}\n", decrypted))?;
        }
        
        self.game_state.add_reputation(10);
        self.game_state.increase_heat(5.0);
        
        Ok(CommandResult::Continue)
    }

    /// Handle inject command
    async fn handle_inject(&mut self, args: Vec<String>) -> Result<CommandResult> {
        if args.is_empty() {
            self.color_scheme.print_error("  [!] Usage: inject <target> <payload_type>\n")?;
            return Ok(CommandResult::Continue);
        }

        let target = &args[0];
        let payload = args.get(1).map(|s| s.as_str()).unwrap_or("trojan");
        
        println!();
        self.color_scheme.print_colored(&format!("  [>] Preparing {} payload for {}...\n", payload, target))?;
        animations::show_processing("Compiling payload", 1500).await?;
        
        self.color_scheme.print_colored("  [>] Establishing connection...\n")?;
        sleep(Duration::from_millis(800)).await;
        
        self.color_scheme.print_colored("  [>] Bypassing security...\n")?;
        sleep(Duration::from_millis(1000)).await;
        
        self.color_scheme.print_colored("  [>] Injecting payload...\n")?;
        animations::show_processing("Injection in progress", 2000).await?;
        
        // Random success/failure
        let success = rand::random::<bool>();
        
        if success {
            self.color_scheme.print_success(&format!("\n  [✓] {} successfully injected into {}\n", payload, target))?;
            self.game_state.add_reputation(15);
            self.game_state.increase_heat(20.0);
        } else {
            self.color_scheme.print_error("\n  [✗] Injection failed - Target secured\n")?;
            self.game_state.increase_heat(10.0);
        }
        
        Ok(CommandResult::Continue)
    }

    /// Handle trace command
    async fn handle_trace(&mut self, args: Vec<String>) -> Result<CommandResult> {
        let target = if args.is_empty() {
            crate::commands::generate_random_ip()
        } else {
            args[0].clone()
        };

        println!();
        self.color_scheme.print_colored(&format!("  [>] Tracing route to {}...\n\n", target))?;
        
        let hops = rand::random::<u8>() % 10 + 5;
        
        for i in 1..=hops {
            let hop_ip = crate::commands::generate_random_ip();
            let latency = rand::random::<u16>() % 150 + 10;
            
            self.color_scheme.print_colored(&format!("  {:2}  ", i))?;
            animations::type_text_effect(&hop_ip, 20, &self.color_scheme).await?;
            self.color_scheme.print_dim(&format!("  [{} ms]", latency))?;
            
            if i == hops {
                self.color_scheme.print_success("  [TARGET REACHED]")?;
            }
            
            println!();
            sleep(Duration::from_millis(300)).await;
        }
        
        println!();
        self.color_scheme.print_success(&format!("  [✓] Trace complete: {} hops to target\n", hops))?;
        
        self.game_state.increase_heat(3.0);
        
        Ok(CommandResult::Continue)
    }

    /// Handle status command
    async fn handle_status(&mut self) -> Result<CommandResult> {
        println!();
        self.color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        self.color_scheme.print_bright("                    AGENT STATUS                               \n")?;
        self.color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        println!();
        
        self.color_scheme.print_colored(&format!("  Agent:      {}\n", self.game_state.username))?;
        self.color_scheme.print_colored(&format!("  Reputation: {}\n", self.game_state.reputation))?;
        
        // Heat level with visual indicator
        let heat_bar = self.create_heat_bar(self.game_state.heat_level);
        self.color_scheme.print_colored("  Heat Level: ")?;
        if self.game_state.heat_level > 75.0 {
            self.color_scheme.print_error(&heat_bar)?;
        } else if self.game_state.heat_level > 50.0 {
            self.color_scheme.print_warning(&heat_bar)?;
        } else {
            self.color_scheme.print_success(&heat_bar)?;
        }
        println!();
        
        self.color_scheme.print_colored(&format!("  Missions:   {} completed\n", self.game_state.missions_completed))?;
        self.color_scheme.print_colored(&format!("  Hacks:      {} successful\n", self.game_state.successful_hacks))?;
        
        println!();
        self.color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        
        Ok(CommandResult::Continue)
    }

    /// Handle mission command
    async fn handle_mission(&mut self, args: Vec<String>) -> Result<CommandResult> {
        println!();
        self.color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        self.color_scheme.print_bright("                    MISSION BRIEFING                           \n")?;
        self.color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        println!();
        
        let missions = vec![
            ("ALPHA-01", "Infiltrate corporate mainframe", "HIGH", 50),
            ("BETA-02", "Decrypt government communications", "MEDIUM", 30),
            ("GAMMA-03", "Deploy ransomware to target", "EXTREME", 100),
            ("DELTA-04", "Extract database from server", "LOW", 20),
        ];
        
        for (id, desc, risk, reward) in missions {
            self.color_scheme.print_bright(&format!("  [{}]\n", id))?;
            self.color_scheme.print_colored(&format!("    Description: {}\n", desc))?;
            
            match risk {
                "EXTREME" => self.color_scheme.print_error(&format!("    Risk: {}\n", risk))?,
                "HIGH" => self.color_scheme.print_warning(&format!("    Risk: {}\n", risk))?,
                _ => self.color_scheme.print_success(&format!("    Risk: {}\n", risk))?,
            }
            
            self.color_scheme.print_colored(&format!("    Reward: {} reputation\n", reward))?;
            println!();
        }
        
        self.color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        self.color_scheme.print_dim("  Type 'mission accept <id>' to accept a mission\n")?;
        
        Ok(CommandResult::Continue)
    }

    /// Handle darkweb command
    async fn handle_darkweb(&mut self, args: Vec<String>) -> Result<CommandResult> {
        println!();
        self.color_scheme.print_colored("  [>] Connecting to dark web...\n")?;
        animations::show_processing("Establishing TOR connection", 2000).await?;
        
        println!();
        self.color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        self.color_scheme.print_bright("              DARK WEB MARKETPLACE                             \n")?;
        self.color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        println!();
        
        let items = vec![
            ("Zero-Day Exploit Kit", "1000"),
            ("Botnet Access (10k nodes)", "5000"),
            ("Database Dump (Fortune 500)", "2500"),
            ("Custom Malware Framework", "3000"),
            ("Stolen Credentials Pack", "500"),
        ];
        
        for (item, price) in items {
            self.color_scheme.print_colored(&format!("  • {}\n", item))?;
            self.color_scheme.print_dim(&format!("    Price: {} credits\n", price))?;
        }
        
        println!();
        self.color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        
        self.game_state.increase_heat(5.0);
        
        Ok(CommandResult::Continue)
    }

    /// Handle firewall command
    async fn handle_firewall(&mut self, args: Vec<String>) -> Result<CommandResult> {
        if args.is_empty() {
            self.color_scheme.print_error("  [!] Usage: firewall <target> [bypass|disable|analyze]\n")?;
            return Ok(CommandResult::Continue);
        }

        let target = &args[0];
        let action = args.get(1).map(|s| s.as_str()).unwrap_or("analyze");
        
        println!();
        self.color_scheme.print_colored(&format!("  [>] Analyzing firewall on {}...\n", target))?;
        animations::show_processing("Detecting firewall rules", 1500).await?;
        
        match action {
            "bypass" => {
                self.color_scheme.print_colored("  [>] Attempting to bypass firewall...\n")?;
                animations::show_processing("Exploiting vulnerabilities", 2000).await?;
                self.color_scheme.print_success("\n  [✓] Firewall bypassed successfully\n")?;
                self.game_state.add_reputation(25);
                self.game_state.increase_heat(30.0);
            }
            "disable" => {
                self.color_scheme.print_colored("  [>] Attempting to disable firewall...\n")?;
                animations::show_processing("Sending kill packets", 2500).await?;
                self.color_scheme.print_warning("\n  [!] Firewall temporarily disabled\n")?;
                self.game_state.add_reputation(30);
                self.game_state.increase_heat(40.0);
            }
            _ => {
                self.color_scheme.print_colored("\n  Firewall Analysis:\n")?;
                self.color_scheme.print_colored("    Type: Next-Gen Enterprise Firewall\n")?;
                self.color_scheme.print_colored("    Rules: 247 active\n")?;
                self.color_scheme.print_colored("    IDS/IPS: Enabled\n")?;
                self.color_scheme.print_warning("    Vulnerabilities: 3 potential weaknesses detected\n")?;
                self.game_state.increase_heat(5.0);
            }
        }
        
        Ok(CommandResult::Continue)
    }

    /// Handle clear command
    async fn handle_clear(&self) -> Result<CommandResult> {
        crate::utils::clear_screen()?;
        Ok(CommandResult::Continue)
    }

    /// Create heat level bar
    fn create_heat_bar(&self, heat: f32) -> String {
        let bar_width = 20;
        let filled = ((heat / 100.0) * bar_width as f32) as usize;
        let empty = bar_width - filled;
        
        let mut bar = String::from("[");
        for _ in 0..filled {
            bar.push('█');
        }
        for _ in 0..empty {
            bar.push('░');
        }
        bar.push_str(&format!("] {:.0}%", heat));
        
        bar
    }

    /// Get current game state
    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }

    /// Update game state
    pub fn update_game_state(&mut self, state: GameState) {
        self.game_state = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_result() {
        let result = CommandResult::Success("Test".to_string());
        assert!(matches!(result, CommandResult::Success(_)));
    }

    #[test]
    fn test_heat_bar_creation() {
        let user = User::new("test".to_string(), "pass").unwrap();
        let handler = CommandHandler::new(&user);
        
        let bar = handler.create_heat_bar(50.0);
        assert!(bar.contains("50%"));
        assert!(bar.contains('█'));
        assert!(bar.contains('░'));
    }
}