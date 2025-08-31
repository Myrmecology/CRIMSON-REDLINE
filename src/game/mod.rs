//! Game state and mechanics for CRIMSON-REDLINE

pub mod state;
pub mod reputation;
pub mod events;

pub use state::{GameState, PlayerStats};
pub use reputation::{ReputationLevel, ReputationManager};
pub use events::{RandomEvent, EventManager};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Save game data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGame {
    pub game_state: GameState,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

impl SaveGame {
    /// Create new save game
    pub fn new(game_state: GameState) -> Self {
        SaveGame {
            game_state,
            timestamp: chrono::Utc::now(),
            version: crate::APP_VERSION.to_string(),
        }
    }

    /// Save to file
    pub fn save(&self) -> Result<()> {
        let save_path = get_save_path()?;
        let data = bincode::serialize(self)?;
        std::fs::write(save_path, data)?;
        Ok(())
    }

    /// Load from file
    pub fn load() -> Result<Option<SaveGame>> {
        let save_path = get_save_path()?;
        
        if !save_path.exists() {
            return Ok(None);
        }

        let data = std::fs::read(save_path)?;
        let save_game: SaveGame = bincode::deserialize(&data)?;
        Ok(Some(save_game))
    }

    /// Delete save file
    pub fn delete() -> Result<()> {
        let save_path = get_save_path()?;
        if save_path.exists() {
            std::fs::remove_file(save_path)?;
        }
        Ok(())
    }
}

/// Get save game file path
fn get_save_path() -> Result<PathBuf> {
    let data_dir = crate::utils::get_data_dir()?;
    Ok(data_dir.join(crate::GAME_STATE_FILE))
}

/// Mission structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub id: String,
    pub name: String,
    pub description: String,
    pub objectives: Vec<Objective>,
    pub reward_reputation: i32,
    pub reward_credits: i32,
    pub difficulty: MissionDifficulty,
    pub time_limit: Option<std::time::Duration>,
    pub is_completed: bool,
    pub is_active: bool,
}

/// Mission objective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    pub id: String,
    pub description: String,
    pub target: Option<String>,
    pub progress: u32,
    pub required: u32,
    pub is_completed: bool,
}

/// Mission difficulty levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MissionDifficulty {
    Trivial,
    Easy,
    Medium,
    Hard,
    Extreme,
    Impossible,
}

impl Mission {
    /// Create a new mission
    pub fn new(
        id: String,
        name: String,
        description: String,
        difficulty: MissionDifficulty,
        reward_reputation: i32,
    ) -> Self {
        Mission {
            id,
            name,
            description,
            objectives: Vec::new(),
            reward_reputation,
            reward_credits: reward_reputation * 10,
            difficulty,
            time_limit: None,
            is_completed: false,
            is_active: false,
        }
    }

    /// Add an objective to the mission
    pub fn add_objective(&mut self, description: String, required: u32) -> &mut Self {
        let id = format!("obj_{}", self.objectives.len() + 1);
        self.objectives.push(Objective {
            id,
            description,
            target: None,
            progress: 0,
            required,
            is_completed: false,
        });
        self
    }

    /// Update objective progress
    pub fn update_objective(&mut self, objective_id: &str, progress: u32) {
        if let Some(obj) = self.objectives.iter_mut().find(|o| o.id == objective_id) {
            obj.progress = obj.progress.saturating_add(progress);
            if obj.progress >= obj.required {
                obj.is_completed = true;
            }
        }

        // Check if all objectives are completed
        if self.objectives.iter().all(|o| o.is_completed) {
            self.is_completed = true;
        }
    }

    /// Get completion percentage
    pub fn get_completion_percentage(&self) -> f32 {
        if self.objectives.is_empty() {
            return 0.0;
        }

        let total_progress: u32 = self.objectives.iter().map(|o| o.progress.min(o.required)).sum();
        let total_required: u32 = self.objectives.iter().map(|o| o.required).sum();

        if total_required == 0 {
            return 0.0;
        }

        (total_progress as f32 / total_required as f32) * 100.0
    }
}

/// Achievement system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub is_unlocked: bool,
    pub unlocked_at: Option<chrono::DateTime<chrono::Utc>>,
    pub rarity: AchievementRarity,
    pub points: u32,
}

/// Achievement rarity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Achievement {
    /// Create a new achievement
    pub fn new(id: String, name: String, description: String, rarity: AchievementRarity) -> Self {
        let points = match rarity {
            AchievementRarity::Common => 10,
            AchievementRarity::Uncommon => 25,
            AchievementRarity::Rare => 50,
            AchievementRarity::Epic => 100,
            AchievementRarity::Legendary => 250,
        };

        Achievement {
            id,
            name,
            description,
            icon: "🏆".to_string(),
            is_unlocked: false,
            unlocked_at: None,
            rarity,
            points,
        }
    }

    /// Unlock the achievement
    pub fn unlock(&mut self) {
        if !self.is_unlocked {
            self.is_unlocked = true;
            self.unlocked_at = Some(chrono::Utc::now());
        }
    }
}

/// Generate default missions
pub fn generate_missions() -> Vec<Mission> {
    let mut missions = Vec::new();

    // Mission 1: First Steps
    let mut mission = Mission::new(
        "INIT-001".to_string(),
        "First Steps".to_string(),
        "Learn the basics of the system".to_string(),
        MissionDifficulty::Trivial,
        10,
    );
    mission.add_objective("Scan a network".to_string(), 1);
    mission.add_objective("Decrypt a file".to_string(), 1);
    missions.push(mission);

    // Mission 2: Network Reconnaissance
    let mut mission = Mission::new(
        "RECON-001".to_string(),
        "Network Reconnaissance".to_string(),
        "Map out a corporate network".to_string(),
        MissionDifficulty::Easy,
        25,
    );
    mission.add_objective("Scan 5 different targets".to_string(), 5);
    mission.add_objective("Identify 3 vulnerabilities".to_string(), 3);
    missions.push(mission);

    // Mission 3: Data Extraction
    let mut mission = Mission::new(
        "DATA-001".to_string(),
        "Data Extraction".to_string(),
        "Extract sensitive data from a secure server".to_string(),
        MissionDifficulty::Medium,
        50,
    );
    mission.add_objective("Exploit a vulnerability".to_string(), 1);
    mission.add_objective("Decrypt 3 files".to_string(), 3);
    mission.add_objective("Maintain heat level below 50%".to_string(), 1);
    missions.push(mission);

    // Mission 4: Corporate Espionage
    let mut mission = Mission::new(
        "CORP-001".to_string(),
        "Corporate Espionage".to_string(),
        "Infiltrate a rival corporation's mainframe".to_string(),
        MissionDifficulty::Hard,
        100,
    );
    mission.add_objective("Bypass 2 firewalls".to_string(), 2);
    mission.add_objective("Successfully exploit 3 systems".to_string(), 3);
    mission.add_objective("Extract database".to_string(), 1);
    mission.time_limit = Some(std::time::Duration::from_secs(600)); // 10 minutes
    missions.push(mission);

    // Mission 5: Ghost Protocol
    let mut mission = Mission::new(
        "GHOST-001".to_string(),
        "Ghost Protocol".to_string(),
        "Complete operations without detection".to_string(),
        MissionDifficulty::Extreme,
        200,
    );
    mission.add_objective("Complete 5 hacks".to_string(), 5);
    mission.add_objective("Keep heat level at 0%".to_string(), 1);
    mission.add_objective("Leave no traces".to_string(), 1);
    missions.push(mission);

    // Mission 6: The Impossible
    let mut mission = Mission::new(
        "IMPOSSIBLE-001".to_string(),
        "The Impossible".to_string(),
        "Hack the unhackable".to_string(),
        MissionDifficulty::Impossible,
        500,
    );
    mission.add_objective("Breach quantum encryption".to_string(), 1);
    mission.add_objective("Defeat AI defense system".to_string(), 1);
    mission.add_objective("Extract the crown jewels".to_string(), 1);
    mission.time_limit = Some(std::time::Duration::from_secs(300)); // 5 minutes
    missions.push(mission);

    missions
}

/// Generate default achievements
pub fn generate_achievements() -> Vec<Achievement> {
    vec![
        Achievement::new(
            "first_login".to_string(),
            "Welcome to the Grid".to_string(),
            "Successfully login for the first time".to_string(),
            AchievementRarity::Common,
        ),
        Achievement::new(
            "first_scan".to_string(),
            "Network Explorer".to_string(),
            "Complete your first network scan".to_string(),
            AchievementRarity::Common,
        ),
        Achievement::new(
            "first_exploit".to_string(),
            "Script Kiddie".to_string(),
            "Successfully exploit your first vulnerability".to_string(),
            AchievementRarity::Common,
        ),
        Achievement::new(
            "decrypt_master".to_string(),
            "Codebreaker".to_string(),
            "Decrypt 50 encrypted files".to_string(),
            AchievementRarity::Uncommon,
        ),
        Achievement::new(
            "stealth_master".to_string(),
            "Ghost in the Machine".to_string(),
            "Complete 10 operations with heat level below 25%".to_string(),
            AchievementRarity::Rare,
        ),
        Achievement::new(
            "reputation_100".to_string(),
            "Notorious".to_string(),
            "Reach 100 reputation points".to_string(),
            AchievementRarity::Uncommon,
        ),
        Achievement::new(
            "reputation_1000".to_string(),
            "Elite Hacker".to_string(),
            "Reach 1000 reputation points".to_string(),
            AchievementRarity::Epic,
        ),
        Achievement::new(
            "perfect_mission".to_string(),
            "Flawless Victory".to_string(),
            "Complete a mission with 100% objectives and 0% heat".to_string(),
            AchievementRarity::Rare,
        ),
        Achievement::new(
            "speed_demon".to_string(),
            "Speed Demon".to_string(),
            "Complete a timed mission with over 50% time remaining".to_string(),
            AchievementRarity::Epic,
        ),
        Achievement::new(
            "master_hacker".to_string(),
            "Master of the Digital Domain".to_string(),
            "Complete all missions and unlock all achievements".to_string(),
            AchievementRarity::Legendary,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mission_creation() {
        let mut mission = Mission::new(
            "TEST-001".to_string(),
            "Test Mission".to_string(),
            "A test mission".to_string(),
            MissionDifficulty::Easy,
            50,
        );
        
        mission.add_objective("Test objective".to_string(), 5);
        assert_eq!(mission.objectives.len(), 1);
        assert_eq!(mission.get_completion_percentage(), 0.0);
        
        mission.update_objective("obj_1", 3);
        assert_eq!(mission.get_completion_percentage(), 60.0);
        
        mission.update_objective("obj_1", 2);
        assert_eq!(mission.get_completion_percentage(), 100.0);
        assert!(mission.is_completed);
    }

    #[test]
    fn test_achievement_unlock() {
        let mut achievement = Achievement::new(
            "test".to_string(),
            "Test Achievement".to_string(),
            "Test description".to_string(),
            AchievementRarity::Common,
        );
        
        assert!(!achievement.is_unlocked);
        achievement.unlock();
        assert!(achievement.is_unlocked);
        assert!(achievement.unlocked_at.is_some());
    }

    #[test]
    fn test_save_game_creation() {
        let game_state = GameState::new("testuser".to_string(), 100);
        let save_game = SaveGame::new(game_state.clone());
        
        assert_eq!(save_game.game_state.username, "testuser");
        assert_eq!(save_game.version, crate::APP_VERSION);
    }
}