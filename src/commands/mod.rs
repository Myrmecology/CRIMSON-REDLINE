//! Command system for CRIMSON-REDLINE terminal

pub mod scanner;
pub mod exploit;
pub mod decrypt;
pub mod handler;

pub use handler::{CommandHandler, CommandResult};

use crate::ui::ColorScheme;
use anyhow::Result;
use std::collections::HashMap;

/// Command registry for all available commands
pub struct CommandRegistry {
    commands: HashMap<String, CommandInfo>,
}

/// Information about a command
#[derive(Clone)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub aliases: Vec<String>,
}

impl CommandRegistry {
    /// Create and populate the command registry
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        
        // Help command
        commands.insert("help".to_string(), CommandInfo {
            name: "help".to_string(),
            description: "Display available commands and their usage".to_string(),
            usage: "help [command]".to_string(),
            aliases: vec!["?".to_string(), "h".to_string()],
        });
        
        // Scan command
        commands.insert("scan".to_string(), CommandInfo {
            name: "scan".to_string(),
            description: "Scan network for targets and vulnerabilities".to_string(),
            usage: "scan [target_ip] [-p ports] [-v verbose]".to_string(),
            aliases: vec!["nmap".to_string(), "recon".to_string()],
        });
        
        // Exploit command
        commands.insert("exploit".to_string(), CommandInfo {
            name: "exploit".to_string(),
            description: "Deploy exploit against identified vulnerability".to_string(),
            usage: "exploit <target> <vulnerability_id>".to_string(),
            aliases: vec!["pwn".to_string(), "attack".to_string()],
        });
        
        // Decrypt command
        commands.insert("decrypt".to_string(), CommandInfo {
            name: "decrypt".to_string(),
            description: "Decrypt intercepted data or files".to_string(),
            usage: "decrypt <encrypted_data> [-k key] [-m method]".to_string(),
            aliases: vec!["decode".to_string(), "decipher".to_string()],
        });
        
        // Inject command
        commands.insert("inject".to_string(), CommandInfo {
            name: "inject".to_string(),
            description: "Inject payload into target system".to_string(),
            usage: "inject <target> <payload_type>".to_string(),
            aliases: vec!["payload".to_string(), "implant".to_string()],
        });
        
        // Trace command
        commands.insert("trace".to_string(), CommandInfo {
            name: "trace".to_string(),
            description: "Trace network route to target".to_string(),
            usage: "trace <target_ip>".to_string(),
            aliases: vec!["traceroute".to_string(), "track".to_string()],
        });
        
        // Status command
        commands.insert("status".to_string(), CommandInfo {
            name: "status".to_string(),
            description: "Display current agent status and statistics".to_string(),
            usage: "status [-v verbose]".to_string(),
            aliases: vec!["stats".to_string(), "info".to_string()],
        });
        
        // Mission command
        commands.insert("mission".to_string(), CommandInfo {
            name: "mission".to_string(),
            description: "Access mission briefings and objectives".to_string(),
            usage: "mission [list|view|accept] [mission_id]".to_string(),
            aliases: vec!["objective".to_string(), "task".to_string()],
        });
        
        // DarkWeb command
        commands.insert("darkweb".to_string(), CommandInfo {
            name: "darkweb".to_string(),
            description: "Access underground marketplace".to_string(),
            usage: "darkweb [browse|buy|sell] [item_id]".to_string(),
            aliases: vec!["market".to_string(), "underground".to_string()],
        });
        
        // Firewall command
        commands.insert("firewall".to_string(), CommandInfo {
            name: "firewall".to_string(),
            description: "Analyze and breach firewall defenses".to_string(),
            usage: "firewall <target> [bypass|disable|analyze]".to_string(),
            aliases: vec!["fw".to_string(), "barrier".to_string()],
        });
        
        // Clear command
        commands.insert("clear".to_string(), CommandInfo {
            name: "clear".to_string(),
            description: "Clear terminal screen".to_string(),
            usage: "clear".to_string(),
            aliases: vec!["cls".to_string(), "cl".to_string()],
        });
        
        // Logout command
        commands.insert("logout".to_string(), CommandInfo {
            name: "logout".to_string(),
            description: "Disconnect from the system".to_string(),
            usage: "logout".to_string(),
            aliases: vec!["exit".to_string(), "quit".to_string(), "disconnect".to_string()],
        });
        
        CommandRegistry { commands }
    }
    
    /// Get command info by name or alias
    pub fn get_command(&self, name: &str) -> Option<&CommandInfo> {
        // Check direct command name
        if let Some(cmd) = self.commands.get(name) {
            return Some(cmd);
        }
        
        // Check aliases
        for cmd in self.commands.values() {
            if cmd.aliases.contains(&name.to_string()) {
                return Some(cmd);
            }
        }
        
        None
    }
    
    /// Get all commands
    pub fn all_commands(&self) -> Vec<&CommandInfo> {
        let mut cmds: Vec<&CommandInfo> = self.commands.values().collect();
        cmds.sort_by(|a, b| a.name.cmp(&b.name));
        cmds
    }
    
    /// Display help for all commands
    pub async fn display_help(&self, color_scheme: &ColorScheme) -> Result<()> {
        println!();
        color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        color_scheme.print_bright("                    AVAILABLE COMMANDS                         \n")?;
        color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        println!();
        
        for cmd in self.all_commands() {
            // Command name
            color_scheme.print_bright(&format!("  {:<12}", cmd.name))?;
            
            // Description
            color_scheme.print_colored(&format!(" - {}\n", cmd.description))?;
            
            // Usage
            color_scheme.print_dim(&format!("               Usage: {}\n", cmd.usage))?;
            
            // Aliases if any
            if !cmd.aliases.is_empty() {
                color_scheme.print_dim(&format!("               Aliases: {}\n", cmd.aliases.join(", ")))?;
            }
            
            println!();
        }
        
        color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        color_scheme.print_dim("  Type 'help <command>' for detailed information about a command\n")?;
        color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
        println!();
        
        Ok(())
    }
    
    /// Display help for a specific command
    pub async fn display_command_help(&self, command: &str, color_scheme: &ColorScheme) -> Result<()> {
        if let Some(cmd) = self.get_command(command) {
            println!();
            color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
            color_scheme.print_bright(&format!("  COMMAND: {}\n", cmd.name.to_uppercase()))?;
            color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
            println!();
            
            color_scheme.print_colored("  Description:\n")?;
            color_scheme.print_secondary(&format!("    {}\n", cmd.description))?;
            println!();
            
            color_scheme.print_colored("  Usage:\n")?;
            color_scheme.print_secondary(&format!("    {}\n", cmd.usage))?;
            println!();
            
            if !cmd.aliases.is_empty() {
                color_scheme.print_colored("  Aliases:\n")?;
                color_scheme.print_secondary(&format!("    {}\n", cmd.aliases.join(", ")))?;
                println!();
            }
            
            color_scheme.print_colored("═══════════════════════════════════════════════════════════════\n")?;
            println!();
        } else {
            color_scheme.print_error(&format!("  [!] Unknown command: {}\n", command))?;
            color_scheme.print_dim("  Type 'help' to see available commands\n")?;
            println!();
        }
        
        Ok(())
    }
}

/// Parse command arguments
pub fn parse_args(input: &str) -> (String, Vec<String>) {
    let parts: Vec<String> = input.split_whitespace().map(String::from).collect();
    
    if parts.is_empty() {
        (String::new(), Vec::new())
    } else {
        (parts[0].clone(), parts[1..].to_vec())
    }
}

/// Generate random IP address
pub fn generate_random_ip() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!(
        "{}.{}.{}.{}",
        rng.gen_range(1..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(1..255)
    )
}

/// Generate random MAC address
pub fn generate_random_mac() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255)
    )
}

/// Generate random hostname
pub fn generate_random_hostname() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    let prefixes = vec![
        "SRV", "WKS", "DB", "WEB", "APP", "FILE", "MAIL", "PROXY",
        "GW", "FW", "ROUTER", "SWITCH", "NODE", "HOST", "CLIENT"
    ];
    
    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let number = rng.gen_range(1..999);
    
    format!("{}-{:03}", prefix, number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_registry() {
        let registry = CommandRegistry::new();
        
        // Test direct command lookup
        assert!(registry.get_command("help").is_some());
        assert!(registry.get_command("scan").is_some());
        
        // Test alias lookup
        assert!(registry.get_command("?").is_some());
        assert!(registry.get_command("nmap").is_some());
        
        // Test non-existent command
        assert!(registry.get_command("nonexistent").is_none());
    }

    #[test]
    fn test_parse_args() {
        let (cmd, args) = parse_args("scan 192.168.1.1 -p 80");
        assert_eq!(cmd, "scan");
        assert_eq!(args, vec!["192.168.1.1", "-p", "80"]);
        
        let (cmd, args) = parse_args("help");
        assert_eq!(cmd, "help");
        assert!(args.is_empty());
    }

    #[test]
    fn test_random_generators() {
        let ip = generate_random_ip();
        assert!(ip.split('.').count() == 4);
        
        let mac = generate_random_mac();
        assert!(mac.split(':').count() == 6);
        
        let hostname = generate_random_hostname();
        assert!(hostname.contains('-'));
    }
}