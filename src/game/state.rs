//! Game state management for CRIMSON-REDLINE

use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Main game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub username: String,
    pub reputation: i32,
    pub heat_level: f32,
    pub credits: i32,
    pub missions_completed: u32,
    pub successful_hacks: u32,
    pub failed_hacks: u32,
    pub total_scans: u32,
    pub files_decrypted: u32,
    pub systems_compromised: u32,
    pub time_played: std::time::Duration,
    pub session_start: chrono::DateTime<chrono::Utc>,
    pub active_missions: Vec<String>,
    pub completed_missions: Vec<String>,
    pub unlocked_tools: Vec<String>,
    pub discovered_exploits: Vec<String>,
    pub network_map: NetworkMap,
}

impl GameState {
    /// Create a new game state
    pub fn new(username: String, starting_reputation: i32) -> Self {
        GameState {
            username,
            reputation: starting_reputation,
            heat_level: 0.0,
            credits: 1000,
            missions_completed: 0,
            successful_hacks: 0,
            failed_hacks: 0,
            total_scans: 0,
            files_decrypted: 0,
            systems_compromised: 0,
            time_played: std::time::Duration::from_secs(0),
            session_start: chrono::Utc::now(),
            active_missions: Vec::new(),
            completed_missions: Vec::new(),
            unlocked_tools: vec!["scan".to_string(), "decrypt".to_string()],
            discovered_exploits: Vec::new(),
            network_map: NetworkMap::new(),
        }
    }

    /// Add reputation
    pub fn add_reputation(&mut self, amount: i32) {
        self.reputation = (self.reputation + amount).max(0);
    }

    /// Increase heat level
    pub fn increase_heat(&mut self, amount: f32) {
        self.heat_level = (self.heat_level + amount).min(100.0);
    }

    /// Decrease heat level (cooldown)
    pub fn decrease_heat(&mut self, amount: f32) {
        self.heat_level = (self.heat_level - amount).max(0.0);
    }

    /// Apply heat decay over time
    pub fn apply_heat_decay(&mut self, decay_rate: f32) {
        self.heat_level = (self.heat_level * decay_rate).max(0.0);
    }

    /// Add credits
    pub fn add_credits(&mut self, amount: i32) {
        self.credits = (self.credits + amount).max(0);
    }

    /// Spend credits
    pub fn spend_credits(&mut self, amount: i32) -> bool {
        if self.credits >= amount {
            self.credits -= amount;
            true
        } else {
            false
        }
    }

    /// Record successful hack
    pub fn record_successful_hack(&mut self) {
        self.successful_hacks += 1;
        self.systems_compromised += 1;
    }

    /// Record failed hack
    pub fn record_failed_hack(&mut self) {
        self.failed_hacks += 1;
    }

    /// Record scan
    pub fn record_scan(&mut self) {
        self.total_scans += 1;
    }

    /// Record decryption
    pub fn record_decryption(&mut self) {
        self.files_decrypted += 1;
    }

    /// Complete mission
    pub fn complete_mission(&mut self, mission_id: String) {
        if let Some(pos) = self.active_missions.iter().position(|x| x == &mission_id) {
            self.active_missions.remove(pos);
            self.completed_missions.push(mission_id);
            self.missions_completed += 1;
        }
    }

    /// Start mission
    pub fn start_mission(&mut self, mission_id: String) {
        if !self.active_missions.contains(&mission_id) && !self.completed_missions.contains(&mission_id) {
            self.active_missions.push(mission_id);
        }
    }

    /// Unlock tool
    pub fn unlock_tool(&mut self, tool: String) {
        if !self.unlocked_tools.contains(&tool) {
            self.unlocked_tools.push(tool);
        }
    }

    /// Discover exploit
    pub fn discover_exploit(&mut self, exploit: String) {
        if !self.discovered_exploits.contains(&exploit) {
            self.discovered_exploits.push(exploit);
        }
    }

    /// Update time played
    pub fn update_time_played(&mut self) {
        let elapsed = chrono::Utc::now() - self.session_start;
        self.time_played += elapsed.to_std().unwrap_or_default();
        self.session_start = chrono::Utc::now();
    }

    /// Get success rate percentage
    pub fn get_success_rate(&self) -> f32 {
        let total = self.successful_hacks + self.failed_hacks;
        if total == 0 {
            0.0
        } else {
            (self.successful_hacks as f32 / total as f32) * 100.0
        }
    }

    /// Check if player is in danger zone (high heat)
    pub fn is_in_danger(&self) -> bool {
        self.heat_level > 75.0
    }

    /// Get player level based on reputation
    pub fn get_level(&self) -> u32 {
        match self.reputation {
            0..=49 => 1,           // Newbie
            50..=149 => 2,         // Script Kiddie
            150..=299 => 3,        // Amateur
            300..=499 => 4,        // Competent
            500..=749 => 5,        // Skilled
            750..=999 => 6,        // Expert
            1000..=1499 => 7,      // Master
            1500..=1999 => 8,      // Elite
            2000..=2999 => 9,      // Legendary
            _ => 10,               // Ghost
        }
    }

    /// Get level title
    pub fn get_level_title(&self) -> &str {
        match self.get_level() {
            1 => "Newbie",
            2 => "Script Kiddie",
            3 => "Amateur Hacker",
            4 => "Competent Hacker",
            5 => "Skilled Hacker",
            6 => "Expert Hacker",
            7 => "Master Hacker",
            8 => "Elite Hacker",
            9 => "Legendary Hacker",
            10 => "Ghost",
            _ => "Unknown",
        }
    }
}

/// Network map for discovered systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMap {
    pub discovered_nodes: Vec<NetworkNode>,
    pub connections: Vec<Connection>,
}

impl NetworkMap {
    /// Create new network map
    pub fn new() -> Self {
        NetworkMap {
            discovered_nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    /// Add discovered node
    pub fn add_node(&mut self, node: NetworkNode) {
        if !self.discovered_nodes.iter().any(|n| n.ip == node.ip) {
            self.discovered_nodes.push(node);
        }
    }

    /// Add connection between nodes
    pub fn add_connection(&mut self, from: String, to: String) {
        let conn = Connection { from: from.clone(), to: to.clone() };
        if !self.connections.contains(&conn) {
            self.connections.push(conn);
        }
    }

    /// Get node by IP
    pub fn get_node(&self, ip: &str) -> Option<&NetworkNode> {
        self.discovered_nodes.iter().find(|n| n.ip == ip)
    }

    /// Get all connected nodes
    pub fn get_connected_nodes(&self, ip: &str) -> Vec<&NetworkNode> {
        let mut connected = Vec::new();
        
        for conn in &self.connections {
            if conn.from == ip {
                if let Some(node) = self.get_node(&conn.to) {
                    connected.push(node);
                }
            } else if conn.to == ip {
                if let Some(node) = self.get_node(&conn.from) {
                    connected.push(node);
                }
            }
        }
        
        connected
    }
}

/// Network node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkNode {
    pub ip: String,
    pub hostname: String,
    pub node_type: NodeType,
    pub is_compromised: bool,
    pub security_level: SecurityLevel,
    pub discovered_at: chrono::DateTime<chrono::Utc>,
}

/// Node types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Server,
    Workstation,
    Router,
    Firewall,
    Database,
    WebServer,
    MailServer,
    DomainController,
    Unknown,
}

/// Security levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    None,
    Low,
    Medium,
    High,
    Maximum,
}

/// Connection between nodes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Connection {
    pub from: String,
    pub to: String,
}

/// Player statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub total_reputation_earned: i32,
    pub total_credits_earned: i32,
    pub total_credits_spent: i32,
    pub highest_heat_level: f32,
    pub longest_session: std::time::Duration,
    pub favorite_command: String,
    pub most_hacked_system: String,
    pub total_data_extracted: u64, // in bytes
    pub unique_exploits_used: u32,
    pub perfect_hacks: u32, // No detection
    pub close_calls: u32, // Heat > 90%
}

impl PlayerStats {
    /// Create new player stats
    pub fn new() -> Self {
        PlayerStats {
            total_reputation_earned: 0,
            total_credits_earned: 0,
            total_credits_spent: 0,
            highest_heat_level: 0.0,
            longest_session: std::time::Duration::from_secs(0),
            favorite_command: String::new(),
            most_hacked_system: String::new(),
            total_data_extracted: 0,
            unique_exploits_used: 0,
            perfect_hacks: 0,
            close_calls: 0,
        }
    }

    /// Update stats from game state
    pub fn update_from_state(&mut self, state: &GameState) {
        self.total_reputation_earned = self.total_reputation_earned.max(state.reputation);
        self.highest_heat_level = self.highest_heat_level.max(state.heat_level);
        
        if state.heat_level == 0.0 && state.successful_hacks > 0 {
            self.perfect_hacks += 1;
        }
        
        if state.heat_level > 90.0 {
            self.close_calls += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_creation() {
        let state = GameState::new("testuser".to_string(), 100);
        assert_eq!(state.username, "testuser");
        assert_eq!(state.reputation, 100);
        assert_eq!(state.heat_level, 0.0);
        assert_eq!(state.credits, 1000);
    }

    #[test]
    fn test_reputation_management() {
        let mut state = GameState::new("testuser".to_string(), 100);
        state.add_reputation(50);
        assert_eq!(state.reputation, 150);
        
        state.add_reputation(-200);
        assert_eq!(state.reputation, 0); // Can't go below 0
    }

    #[test]
    fn test_heat_management() {
        let mut state = GameState::new("testuser".to_string(), 0);
        state.increase_heat(50.0);
        assert_eq!(state.heat_level, 50.0);
        
        state.increase_heat(60.0);
        assert_eq!(state.heat_level, 100.0); // Capped at 100
        
        state.decrease_heat(30.0);
        assert_eq!(state.heat_level, 70.0);
        
        state.apply_heat_decay(0.5);
        assert_eq!(state.heat_level, 35.0);
    }

    #[test]
    fn test_level_calculation() {
        let mut state = GameState::new("testuser".to_string(), 0);
        assert_eq!(state.get_level(), 1);
        assert_eq!(state.get_level_title(), "Newbie");
        
        state.reputation = 500;
        assert_eq!(state.get_level(), 5);
        assert_eq!(state.get_level_title(), "Skilled Hacker");
        
        state.reputation = 2000;
        assert_eq!(state.get_level(), 9);
        assert_eq!(state.get_level_title(), "Legendary Hacker");
    }

    #[test]
    fn test_success_rate() {
        let mut state = GameState::new("testuser".to_string(), 0);
        assert_eq!(state.get_success_rate(), 0.0);
        
        state.successful_hacks = 7;
        state.failed_hacks = 3;
        assert_eq!(state.get_success_rate(), 70.0);
    }

    #[test]
    fn test_network_map() {
        let mut map = NetworkMap::new();
        
        let node1 = NetworkNode {
            ip: "192.168.1.1".to_string(),
            hostname: "router".to_string(),
            node_type: NodeType::Router,
            is_compromised: false,
            security_level: SecurityLevel::Medium,
            discovered_at: chrono::Utc::now(),
        };
        
        let node2 = NetworkNode {
            ip: "192.168.1.10".to_string(),
            hostname: "server".to_string(),
            node_type: NodeType::Server,
            is_compromised: false,
            security_level: SecurityLevel::High,
            discovered_at: chrono::Utc::now(),
        };
        
        map.add_node(node1);
        map.add_node(node2);
        map.add_connection("192.168.1.1".to_string(), "192.168.1.10".to_string());
        
        assert_eq!(map.discovered_nodes.len(), 2);
        assert_eq!(map.connections.len(), 1);
        
        let connected = map.get_connected_nodes("192.168.1.1");
        assert_eq!(connected.len(), 1);
        assert_eq!(connected[0].ip, "192.168.1.10");
    }
}