//! Reputation system for CRIMSON-REDLINE

use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Reputation manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationManager {
    pub current_reputation: i32,
    pub lifetime_reputation: i32,
    pub level: ReputationLevel,
    pub multiplier: f32,
    pub streak: u32,
    pub last_action: Option<chrono::DateTime<chrono::Utc>>,
}

impl ReputationManager {
    /// Create new reputation manager
    pub fn new(starting_reputation: i32) -> Self {
        let level = ReputationLevel::from_reputation(starting_reputation);
        ReputationManager {
            current_reputation: starting_reputation,
            lifetime_reputation: starting_reputation,
            level,
            multiplier: 1.0,
            streak: 0,
            last_action: None,
        }
    }

    /// Add reputation with multipliers
    pub fn add_reputation(&mut self, base_amount: i32) -> i32 {
        let final_amount = (base_amount as f32 * self.multiplier) as i32;
        self.current_reputation += final_amount;
        
        if final_amount > 0 {
            self.lifetime_reputation += final_amount;
        }
        
        // Update level
        self.level = ReputationLevel::from_reputation(self.current_reputation);
        
        // Update streak
        self.update_streak();
        
        final_amount
    }

    /// Remove reputation (penalties)
    pub fn remove_reputation(&mut self, amount: i32) {
        self.current_reputation = (self.current_reputation - amount).max(0);
        self.level = ReputationLevel::from_reputation(self.current_reputation);
        self.reset_streak();
    }

    /// Update action streak
    fn update_streak(&mut self) {
        let now = chrono::Utc::now();
        
        if let Some(last) = self.last_action {
            let duration = now - last;
            
            // If action within 5 minutes, increase streak
            if duration.num_seconds() < 300 {
                self.streak += 1;
                
                // Update multiplier based on streak
                self.multiplier = match self.streak {
                    0..=4 => 1.0,
                    5..=9 => 1.1,
                    10..=19 => 1.25,
                    20..=29 => 1.5,
                    30..=49 => 1.75,
                    _ => 2.0,
                };
            } else {
                self.reset_streak();
            }
        }
        
        self.last_action = Some(now);
    }

    /// Reset streak
    fn reset_streak(&mut self) {
        self.streak = 0;
        self.multiplier = 1.0;
    }

    /// Get bonus description
    pub fn get_streak_bonus_description(&self) -> String {
        match self.streak {
            0..=4 => "No Streak".to_string(),
            5..=9 => "Hot Streak! (+10% bonus)".to_string(),
            10..=19 => "On Fire! (+25% bonus)".to_string(),
            20..=29 => "Unstoppable! (+50% bonus)".to_string(),
            30..=49 => "Legendary! (+75% bonus)".to_string(),
            _ => "GODLIKE! (+100% bonus)".to_string(),
        }
    }

    /// Get reputation needed for next level
    pub fn reputation_to_next_level(&self) -> Option<i32> {
        self.level.next_level_requirement().map(|req| req - self.current_reputation)
    }

    /// Get reputation progress percentage
    pub fn level_progress(&self) -> f32 {
        let current_req = self.level.reputation_requirement();
        let next_req = self.level.next_level_requirement().unwrap_or(current_req + 1000);
        
        let progress = self.current_reputation - current_req;
        let total = next_req - current_req;
        
        (progress as f32 / total as f32 * 100.0).min(100.0).max(0.0)
    }
}

/// Reputation levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReputationLevel {
    Nobody,           // 0-49
    Wannabe,          // 50-149
    ScriptKiddie,     // 150-299
    Amateur,          // 300-499
    Competent,        // 500-749
    Skilled,          // 750-999
    Expert,           // 1000-1499
    Master,           // 1500-1999
    Elite,            // 2000-2999
    Legendary,        // 3000-4999
    Mythical,         // 5000+
}

impl ReputationLevel {
    /// Get level from reputation value
    pub fn from_reputation(reputation: i32) -> Self {
        match reputation {
            0..=49 => ReputationLevel::Nobody,
            50..=149 => ReputationLevel::Wannabe,
            150..=299 => ReputationLevel::ScriptKiddie,
            300..=499 => ReputationLevel::Amateur,
            500..=749 => ReputationLevel::Competent,
            750..=999 => ReputationLevel::Skilled,
            1000..=1499 => ReputationLevel::Expert,
            1500..=1999 => ReputationLevel::Master,
            2000..=2999 => ReputationLevel::Elite,
            3000..=4999 => ReputationLevel::Legendary,
            _ => ReputationLevel::Mythical,
        }
    }

    /// Get reputation requirement for this level
    pub fn reputation_requirement(&self) -> i32 {
        match self {
            ReputationLevel::Nobody => 0,
            ReputationLevel::Wannabe => 50,
            ReputationLevel::ScriptKiddie => 150,
            ReputationLevel::Amateur => 300,
            ReputationLevel::Competent => 500,
            ReputationLevel::Skilled => 750,
            ReputationLevel::Expert => 1000,
            ReputationLevel::Master => 1500,
            ReputationLevel::Elite => 2000,
            ReputationLevel::Legendary => 3000,
            ReputationLevel::Mythical => 5000,
        }
    }

    /// Get next level requirement
    pub fn next_level_requirement(&self) -> Option<i32> {
        match self {
            ReputationLevel::Nobody => Some(50),
            ReputationLevel::Wannabe => Some(150),
            ReputationLevel::ScriptKiddie => Some(300),
            ReputationLevel::Amateur => Some(500),
            ReputationLevel::Competent => Some(750),
            ReputationLevel::Skilled => Some(1000),
            ReputationLevel::Expert => Some(1500),
            ReputationLevel::Master => Some(2000),
            ReputationLevel::Elite => Some(3000),
            ReputationLevel::Legendary => Some(5000),
            ReputationLevel::Mythical => None,
        }
    }

    /// Get level display name
    pub fn display_name(&self) -> &str {
        match self {
            ReputationLevel::Nobody => "Nobody",
            ReputationLevel::Wannabe => "Wannabe",
            ReputationLevel::ScriptKiddie => "Script Kiddie",
            ReputationLevel::Amateur => "Amateur Hacker",
            ReputationLevel::Competent => "Competent Hacker",
            ReputationLevel::Skilled => "Skilled Hacker",
            ReputationLevel::Expert => "Expert Hacker",
            ReputationLevel::Master => "Master Hacker",
            ReputationLevel::Elite => "Elite Hacker",
            ReputationLevel::Legendary => "Legendary Hacker",
            ReputationLevel::Mythical => "Mythical Hacker",
        }
    }

    /// Get level color name for display
    pub fn color_name(&self) -> &str {
        match self {
            ReputationLevel::Nobody => "dim",
            ReputationLevel::Wannabe => "normal",
            ReputationLevel::ScriptKiddie => "normal",
            ReputationLevel::Amateur => "secondary",
            ReputationLevel::Competent => "secondary",
            ReputationLevel::Skilled => "bright",
            ReputationLevel::Expert => "bright",
            ReputationLevel::Master => "success",
            ReputationLevel::Elite => "warning",
            ReputationLevel::Legendary => "error",
            ReputationLevel::Mythical => "error",
        }
    }

    /// Get perks for this level
    pub fn get_perks(&self) -> Vec<String> {
        let mut perks = Vec::new();
        
        match self {
            ReputationLevel::Nobody => {
                perks.push("Basic commands unlocked".to_string());
            }
            ReputationLevel::Wannabe => {
                perks.push("Access to simple exploits".to_string());
                perks.push("+5% reputation bonus".to_string());
            }
            ReputationLevel::ScriptKiddie => {
                perks.push("Intermediate exploits unlocked".to_string());
                perks.push("+10% reputation bonus".to_string());
                perks.push("Heat decay increased by 10%".to_string());
            }
            ReputationLevel::Amateur => {
                perks.push("Advanced scanning tools".to_string());
                perks.push("+15% reputation bonus".to_string());
                perks.push("Access to underground markets".to_string());
            }
            ReputationLevel::Competent => {
                perks.push("Professional exploit kit".to_string());
                perks.push("+20% reputation bonus".to_string());
                perks.push("Heat decay increased by 20%".to_string());
                perks.push("Mission reward bonus +10%".to_string());
            }
            ReputationLevel::Skilled => {
                perks.push("Elite tools unlocked".to_string());
                perks.push("+25% reputation bonus".to_string());
                perks.push("Stealth mode available".to_string());
            }
            ReputationLevel::Expert => {
                perks.push("Zero-day exploits access".to_string());
                perks.push("+30% reputation bonus".to_string());
                perks.push("Heat decay increased by 30%".to_string());
                perks.push("Mission reward bonus +20%".to_string());
            }
            ReputationLevel::Master => {
                perks.push("Custom exploit development".to_string());
                perks.push("+40% reputation bonus".to_string());
                perks.push("Advanced evasion techniques".to_string());
            }
            ReputationLevel::Elite => {
                perks.push("Quantum decryption tools".to_string());
                perks.push("+50% reputation bonus".to_string());
                perks.push("Heat decay increased by 40%".to_string());
                perks.push("Mission reward bonus +30%".to_string());
            }
            ReputationLevel::Legendary => {
                perks.push("AI-assisted hacking".to_string());
                perks.push("+75% reputation bonus".to_string());
                perks.push("Near-invisible operations".to_string());
                perks.push("All tools maximized".to_string());
            }
            ReputationLevel::Mythical => {
                perks.push("God mode activated".to_string());
                perks.push("+100% reputation bonus".to_string());
                perks.push("Instant heat decay".to_string());
                perks.push("All content unlocked".to_string());
                perks.push("Fear and respect of the digital underground".to_string());
            }
        }
        
        perks
    }
}

/// Reputation events that grant or remove reputation
#[derive(Debug, Clone)]
pub struct ReputationEvent {
    pub event_type: ReputationEventType,
    pub base_amount: i32,
    pub description: String,
}

/// Types of reputation events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReputationEventType {
    // Positive events
    SuccessfulHack,
    VulnerabilityDiscovered,
    SystemCompromised,
    DataExtracted,
    MissionCompleted,
    PerfectOperation,
    FirstBlood,
    
    // Negative events
    DetectionTriggered,
    HackFailed,
    SystemLocked,
    MissionFailed,
    TraceCompleted,
}

impl ReputationEvent {
    /// Create reputation event
    pub fn new(event_type: ReputationEventType) -> Self {
        let (base_amount, description) = match event_type {
            ReputationEventType::SuccessfulHack => (10, "Successful hack completed"),
            ReputationEventType::VulnerabilityDiscovered => (5, "New vulnerability discovered"),
            ReputationEventType::SystemCompromised => (15, "System successfully compromised"),
            ReputationEventType::DataExtracted => (8, "Sensitive data extracted"),
            ReputationEventType::MissionCompleted => (25, "Mission completed"),
            ReputationEventType::PerfectOperation => (30, "Perfect operation - no detection"),
            ReputationEventType::FirstBlood => (50, "First successful hack on new target"),
            ReputationEventType::DetectionTriggered => (-5, "Detection systems triggered"),
            ReputationEventType::HackFailed => (-10, "Hack attempt failed"),
            ReputationEventType::SystemLocked => (-15, "Locked out of system"),
            ReputationEventType::MissionFailed => (-25, "Mission failed"),
            ReputationEventType::TraceCompleted => (-20, "Traced back to origin"),
        };
        
        ReputationEvent {
            event_type,
            base_amount,
            description: description.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reputation_levels() {
        assert_eq!(ReputationLevel::from_reputation(0), ReputationLevel::Nobody);
        assert_eq!(ReputationLevel::from_reputation(100), ReputationLevel::Wannabe);
        assert_eq!(ReputationLevel::from_reputation(500), ReputationLevel::Competent);
        assert_eq!(ReputationLevel::from_reputation(5000), ReputationLevel::Mythical);
    }

    #[test]
    fn test_reputation_manager() {
        let mut manager = ReputationManager::new(100);
        assert_eq!(manager.level, ReputationLevel::Wannabe);
        
        // Test adding reputation
        let added = manager.add_reputation(100);
        assert_eq!(added, 100);
        assert_eq!(manager.current_reputation, 200);
        assert_eq!(manager.level, ReputationLevel::ScriptKiddie);
        
        // Test removing reputation
        manager.remove_reputation(150);
        assert_eq!(manager.current_reputation, 50);
        assert_eq!(manager.level, ReputationLevel::Wannabe);
    }

    #[test]
    fn test_streak_multiplier() {
        let mut manager = ReputationManager::new(0);
        
        // Build streak
        for _ in 0..5 {
            manager.update_streak();
        }
        assert_eq!(manager.streak, 5);
        assert_eq!(manager.multiplier, 1.1);
        
        // Test multiplier effect
        let added = manager.add_reputation(100);
        assert_eq!(added, 110); // 100 * 1.1
    }

    #[test]
    fn test_level_progress() {
        let mut manager = ReputationManager::new(75);
        // 75 reputation: Wannabe level (50-149)
        // Progress: (75-50)/(150-50) = 25/100 = 25%
        assert_eq!(manager.level_progress(), 25.0);
        
        manager.current_reputation = 125;
        // Progress: (125-50)/(150-50) = 75/100 = 75%
        assert_eq!(manager.level_progress(), 75.0);
    }

    #[test]
    fn test_reputation_events() {
        let event = ReputationEvent::new(ReputationEventType::SuccessfulHack);
        assert_eq!(event.base_amount, 10);
        
        let negative_event = ReputationEvent::new(ReputationEventType::HackFailed);
        assert_eq!(negative_event.base_amount, -10);
    }
}