//! Random events system for CRIMSON-REDLINE

use crate::ui::ColorScheme;
use anyhow::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Event manager for random events
#[derive(Debug, Clone)]
pub struct EventManager {
    pub active_events: Vec<RandomEvent>,
    pub event_history: Vec<RandomEvent>,
    pub event_chance: f32,
    pub last_event_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl EventManager {
    /// Create new event manager
    pub fn new() -> Self {
        EventManager {
            active_events: Vec::new(),
            event_history: Vec::new(),
            event_chance: 0.1, // 10% chance per action
            last_event_time: None,
        }
    }

    /// Check if a random event should trigger
    pub fn should_trigger_event(&self) -> bool {
        let mut rng = rand::thread_rng();
        rng.gen::<f32>() < self.event_chance
    }

    /// Generate a random event based on current game state
    pub fn generate_event(&mut self, heat_level: f32, reputation: i32) -> Option<RandomEvent> {
        if !self.should_trigger_event() {
            return None;
        }

        let mut rng = rand::thread_rng();
        
        // Determine event category based on game state
        let event = if heat_level > 75.0 {
            // High heat - more dangerous events
            self.generate_high_heat_event()
        } else if reputation > 1000 {
            // High reputation - special opportunities
            self.generate_high_reputation_event()
        } else if rng.gen::<f32>() < 0.5 {
            // Random opportunity
            self.generate_opportunity_event()
        } else {
            // Random threat
            self.generate_threat_event()
        };

        self.last_event_time = Some(chrono::Utc::now());
        self.event_history.push(event.clone());
        self.active_events.push(event.clone());
        
        Some(event)
    }

    /// Generate high heat event
    fn generate_high_heat_event(&self) -> RandomEvent {
        let mut rng = rand::thread_rng();
        let events = vec![
            RandomEvent {
                id: "trace_initiated".to_string(),
                title: "TRACE INITIATED".to_string(),
                description: "Security forces are attempting to trace your location!".to_string(),
                event_type: EventType::Threat,
                severity: EventSeverity::Critical,
                choices: vec![
                    EventChoice {
                        label: "Deploy countermeasures".to_string(),
                        outcome: EventOutcome::ReduceHeat(30.0),
                        cost: Some(EventCost::Credits(100)),
                    },
                    EventChoice {
                        label: "Go dark immediately".to_string(),
                        outcome: EventOutcome::ReduceHeat(50.0),
                        cost: Some(EventCost::Reputation(20)),
                    },
                    EventChoice {
                        label: "Risk it".to_string(),
                        outcome: EventOutcome::IncreaseHeat(20.0),
                        cost: None,
                    },
                ],
                time_limit: Some(std::time::Duration::from_secs(30)),
            },
            RandomEvent {
                id: "system_lockdown".to_string(),
                title: "SYSTEM LOCKDOWN".to_string(),
                description: "Target system is initiating emergency lockdown procedures!".to_string(),
                event_type: EventType::Threat,
                severity: EventSeverity::High,
                choices: vec![
                    EventChoice {
                        label: "Force override".to_string(),
                        outcome: EventOutcome::MaintainAccess,
                        cost: Some(EventCost::Heat(25.0)),
                    },
                    EventChoice {
                        label: "Extract and flee".to_string(),
                        outcome: EventOutcome::SafeExit,
                        cost: None,
                    },
                ],
                time_limit: Some(std::time::Duration::from_secs(20)),
            },
        ];
        
        events[rng.gen_range(0..events.len())].clone()
    }

    /// Generate high reputation event
    fn generate_high_reputation_event(&self) -> RandomEvent {
        let mut rng = rand::thread_rng();
        let events = vec![
            RandomEvent {
                id: "elite_invitation".to_string(),
                title: "ELITE INVITATION".to_string(),
                description: "You've been invited to join an elite hacker collective".to_string(),
                event_type: EventType::Opportunity,
                severity: EventSeverity::Low,
                choices: vec![
                    EventChoice {
                        label: "Accept invitation".to_string(),
                        outcome: EventOutcome::UnlockContent("elite_tools".to_string()),
                        cost: Some(EventCost::Reputation(50)),
                    },
                    EventChoice {
                        label: "Decline respectfully".to_string(),
                        outcome: EventOutcome::GainReputation(10),
                        cost: None,
                    },
                ],
                time_limit: None,
            },
            RandomEvent {
                id: "black_market_deal".to_string(),
                title: "BLACK MARKET OPPORTUNITY".to_string(),
                description: "A mysterious contact offers rare zero-day exploits".to_string(),
                event_type: EventType::Opportunity,
                severity: EventSeverity::Medium,
                choices: vec![
                    EventChoice {
                        label: "Purchase exploits".to_string(),
                        outcome: EventOutcome::UnlockContent("zero_day_pack".to_string()),
                        cost: Some(EventCost::Credits(500)),
                    },
                    EventChoice {
                        label: "Negotiate better price".to_string(),
                        outcome: EventOutcome::Special("negotiation".to_string()),
                        cost: Some(EventCost::Reputation(10)),
                    },
                    EventChoice {
                        label: "Report to authorities".to_string(),
                        outcome: EventOutcome::ReduceHeat(20.0),
                        cost: Some(EventCost::Reputation(30)),
                    },
                ],
                time_limit: Some(std::time::Duration::from_secs(60)),
            },
        ];
        
        events[rng.gen_range(0..events.len())].clone()
    }

    /// Generate opportunity event
    fn generate_opportunity_event(&self) -> RandomEvent {
        let mut rng = rand::thread_rng();
        let events = vec![
            RandomEvent {
                id: "vulnerable_system".to_string(),
                title: "VULNERABLE SYSTEM DETECTED".to_string(),
                description: "Scans reveal a highly vulnerable system with valuable data".to_string(),
                event_type: EventType::Opportunity,
                severity: EventSeverity::Low,
                choices: vec![
                    EventChoice {
                        label: "Exploit immediately".to_string(),
                        outcome: EventOutcome::GainCredits(200),
                        cost: Some(EventCost::Heat(15.0)),
                    },
                    EventChoice {
                        label: "Document and save for later".to_string(),
                        outcome: EventOutcome::UnlockContent("saved_target".to_string()),
                        cost: None,
                    },
                ],
                time_limit: Some(std::time::Duration::from_secs(45)),
            },
            RandomEvent {
                id: "data_cache".to_string(),
                title: "ENCRYPTED DATA CACHE".to_string(),
                description: "You've discovered an encrypted data cache during your scan".to_string(),
                event_type: EventType::Opportunity,
                severity: EventSeverity::Low,
                choices: vec![
                    EventChoice {
                        label: "Decrypt now".to_string(),
                        outcome: EventOutcome::GainCredits(100),
                        cost: Some(EventCost::Time(30)),
                    },
                    EventChoice {
                        label: "Download for later".to_string(),
                        outcome: EventOutcome::Special("encrypted_file".to_string()),
                        cost: None,
                    },
                ],
                time_limit: None,
            },
            RandomEvent {
                id: "backdoor_found".to_string(),
                title: "BACKDOOR DISCOVERED".to_string(),
                description: "You've found an existing backdoor in the system".to_string(),
                event_type: EventType::Opportunity,
                severity: EventSeverity::Medium,
                choices: vec![
                    EventChoice {
                        label: "Use the backdoor".to_string(),
                        outcome: EventOutcome::MaintainAccess,
                        cost: None,
                    },
                    EventChoice {
                        label: "Replace with your own".to_string(),
                        outcome: EventOutcome::Special("persistent_access".to_string()),
                        cost: Some(EventCost::Heat(10.0)),
                    },
                    EventChoice {
                        label: "Report and patch".to_string(),
                        outcome: EventOutcome::GainReputation(25),
                        cost: None,
                    },
                ],
                time_limit: None,
            },
        ];
        
        events[rng.gen_range(0..events.len())].clone()
    }

    /// Generate threat event
    fn generate_threat_event(&self) -> RandomEvent {
        let mut rng = rand::thread_rng();
        let events = vec![
            RandomEvent {
                id: "honeypot".to_string(),
                title: "HONEYPOT DETECTED".to_string(),
                description: "This system appears to be a honeypot trap!".to_string(),
                event_type: EventType::Threat,
                severity: EventSeverity::High,
                choices: vec![
                    EventChoice {
                        label: "Abort immediately".to_string(),
                        outcome: EventOutcome::SafeExit,
                        cost: Some(EventCost::Reputation(5)),
                    },
                    EventChoice {
                        label: "Leave false trail".to_string(),
                        outcome: EventOutcome::ReduceHeat(10.0),
                        cost: Some(EventCost::Credits(50)),
                    },
                    EventChoice {
                        label: "Turn it to your advantage".to_string(),
                        outcome: EventOutcome::Special("honeypot_reversed".to_string()),
                        cost: Some(EventCost::Heat(30.0)),
                    },
                ],
                time_limit: Some(std::time::Duration::from_secs(15)),
            },
            RandomEvent {
                id: "rival_hacker".to_string(),
                title: "RIVAL HACKER DETECTED".to_string(),
                description: "Another hacker is targeting the same system!".to_string(),
                event_type: EventType::Threat,
                severity: EventSeverity::Medium,
                choices: vec![
                    EventChoice {
                        label: "Race to the prize".to_string(),
                        outcome: EventOutcome::Special("race_rival".to_string()),
                        cost: Some(EventCost::Heat(20.0)),
                    },
                    EventChoice {
                        label: "Collaborate".to_string(),
                        outcome: EventOutcome::GainReputation(15),
                        cost: None,
                    },
                    EventChoice {
                        label: "Sabotage their attempt".to_string(),
                        outcome: EventOutcome::Special("sabotage".to_string()),
                        cost: Some(EventCost::Reputation(10)),
                    },
                ],
                time_limit: Some(std::time::Duration::from_secs(30)),
            },
            RandomEvent {
                id: "ai_defense".to_string(),
                title: "AI DEFENSE SYSTEM".to_string(),
                description: "An advanced AI is defending this system!".to_string(),
                event_type: EventType::Threat,
                severity: EventSeverity::Critical,
                choices: vec![
                    EventChoice {
                        label: "Engage in cyber warfare".to_string(),
                        outcome: EventOutcome::Special("ai_battle".to_string()),
                        cost: Some(EventCost::Heat(40.0)),
                    },
                    EventChoice {
                        label: "Attempt to confuse it".to_string(),
                        outcome: EventOutcome::MaintainAccess,
                        cost: Some(EventCost::Credits(150)),
                    },
                    EventChoice {
                        label: "Tactical retreat".to_string(),
                        outcome: EventOutcome::SafeExit,
                        cost: None,
                    },
                ],
                time_limit: Some(std::time::Duration::from_secs(20)),
            },
        ];
        
        events[rng.gen_range(0..events.len())].clone()
    }

    /// Handle event choice
    pub async fn handle_choice(
        &mut self,
        event_id: &str,
        choice_index: usize,
        game_state: &mut crate::game::GameState,
        color_scheme: &ColorScheme,
    ) -> Result<()> {
        if let Some(event_idx) = self.active_events.iter().position(|e| e.id == event_id) {
            let event = self.active_events.remove(event_idx);
            
            if choice_index < event.choices.len() {
                let choice = &event.choices[choice_index];
                
                // Apply cost
                if let Some(ref cost) = choice.cost {
                    match cost {
                        EventCost::Credits(amount) => {
                            if !game_state.spend_credits(*amount) {
                                color_scheme.print_error("  [!] Insufficient credits!\n")?;
                                return Ok(());
                            }
                        }
                        EventCost::Reputation(amount) => {
                            game_state.add_reputation(-(*amount as i32));
                        }
                        EventCost::Heat(amount) => {
                            game_state.increase_heat(*amount);
                        }
                        EventCost::Time(seconds) => {
                            color_scheme.print_colored(&format!("  [>] Waiting {} seconds...\n", seconds))?;
                            tokio::time::sleep(std::time::Duration::from_secs(*seconds as u64)).await;
                        }
                    }
                }
                
                // Apply outcome
                match &choice.outcome {
                    EventOutcome::GainCredits(amount) => {
                        game_state.add_credits(*amount);
                        color_scheme.print_success(&format!("  [+] Gained {} credits!\n", amount))?;
                    }
                    EventOutcome::GainReputation(amount) => {
                        game_state.add_reputation(*amount);
                        color_scheme.print_success(&format!("  [+] Gained {} reputation!\n", amount))?;
                    }
                    EventOutcome::ReduceHeat(amount) => {
                        game_state.decrease_heat(*amount);
                        color_scheme.print_success(&format!("  [+] Heat reduced by {}%!\n", amount))?;
                    }
                    EventOutcome::IncreaseHeat(amount) => {
                        game_state.increase_heat(*amount);
                        color_scheme.print_warning(&format!("  [!] Heat increased by {}%!\n", amount))?;
                    }
                    EventOutcome::UnlockContent(content) => {
                        game_state.unlock_tool(content.clone());
                        color_scheme.print_success(&format!("  [+] Unlocked: {}!\n", content))?;
                    }
                    EventOutcome::MaintainAccess => {
                        color_scheme.print_success("  [+] Access maintained!\n")?;
                    }
                    EventOutcome::SafeExit => {
                        color_scheme.print_success("  [+] Safely exited!\n")?;
                    }
                    EventOutcome::Special(action) => {
                        self.handle_special_outcome(action, game_state, color_scheme).await?;
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Handle special event outcomes
    async fn handle_special_outcome(
        &self,
        action: &str,
        game_state: &mut crate::game::GameState,
        color_scheme: &ColorScheme,
    ) -> Result<()> {
        match action {
            "negotiation" => {
                color_scheme.print_colored("  [>] Negotiating price...\n")?;
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                
                let mut rng = rand::thread_rng();
                if rng.gen::<f32>() > 0.5 {
                    color_scheme.print_success("  [+] Negotiation successful! 50% discount obtained!\n")?;
                    game_state.add_credits(250);
                } else {
                    color_scheme.print_warning("  [!] Negotiation failed. Seller vanished.\n")?;
                }
            }
            "race_rival" => {
                color_scheme.print_colored("  [>] Racing against rival hacker...\n")?;
                crate::ui::animations::show_processing("Competing for access", 3000).await?;
                
                let mut rng = rand::thread_rng();
                if rng.gen::<f32>() > 0.4 {
                    color_scheme.print_success("  [+] Victory! You beat the rival!\n")?;
                    game_state.add_reputation(30);
                    game_state.add_credits(300);
                } else {
                    color_scheme.print_error("  [✗] The rival was faster!\n")?;
                    game_state.add_reputation(-10);
                }
            }
            "ai_battle" => {
                color_scheme.print_colored("  [>] Engaging AI defense system...\n")?;
                crate::ui::animations::show_processing("Cyber warfare in progress", 5000).await?;
                
                let mut rng = rand::thread_rng();
                if rng.gen::<f32>() > 0.3 {
                    color_scheme.print_success("  [+] AI defeated! System compromised!\n")?;
                    game_state.add_reputation(100);
                    game_state.discover_exploit("AI_Slayer".to_string());
                } else {
                    color_scheme.print_error("  [✗] AI victorious. Connection terminated.\n")?;
                    game_state.increase_heat(50.0);
                }
            }
            _ => {
                color_scheme.print_colored(&format!("  [>] Special action: {}\n", action))?;
            }
        }
        
        Ok(())
    }
}

/// Random event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomEvent {
    pub id: String,
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub severity: EventSeverity,
    pub choices: Vec<EventChoice>,
    pub time_limit: Option<std::time::Duration>,
}

/// Event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Opportunity,
    Threat,
    Neutral,
}

/// Event severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Event choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventChoice {
    pub label: String,
    pub outcome: EventOutcome,
    pub cost: Option<EventCost>,
}

/// Event outcomes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventOutcome {
    GainCredits(i32),
    GainReputation(i32),
    ReduceHeat(f32),
    IncreaseHeat(f32),
    UnlockContent(String),
    MaintainAccess,
    SafeExit,
    Special(String),
}

/// Event costs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventCost {
    Credits(i32),
    Reputation(i32),
    Heat(f32),
    Time(u32), // seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_manager_creation() {
        let manager = EventManager::new();
        assert!(manager.active_events.is_empty());
        assert_eq!(manager.event_chance, 0.1);
    }

    #[test]
    fn test_event_generation() {
        let mut manager = EventManager::new();
        manager.event_chance = 1.0; // Force event generation
        
        let event = manager.generate_event(50.0, 500);
        assert!(event.is_some());
        assert_eq!(manager.active_events.len(), 1);
        assert_eq!(manager.event_history.len(), 1);
    }

    #[test]
    fn test_random_event_structure() {
        let event = RandomEvent {
            id: "test_event".to_string(),
            title: "Test Event".to_string(),
            description: "This is a test".to_string(),
            event_type: EventType::Neutral,
            severity: EventSeverity::Low,
            choices: vec![
                EventChoice {
                    label: "Option 1".to_string(),
                    outcome: EventOutcome::GainCredits(100),
                    cost: None,
                },
            ],
            time_limit: None,
        };
        
        assert_eq!(event.id, "test_event");
        assert_eq!(event.choices.len(), 1);
    }
}