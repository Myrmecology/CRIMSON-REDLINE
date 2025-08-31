//! User storage and persistence for CRIMSON-REDLINE

use super::User;
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::fs;

/// Storage structure for all users
#[derive(Debug, Serialize, Deserialize, Default)]
struct UserDatabase {
    users: HashMap<String, User>,
    version: u32,
}

/// User storage handler
pub struct UserStorage {
    db_path: PathBuf,
    database: UserDatabase,
}

impl UserStorage {
    /// Create a new user storage instance
    pub fn new() -> Result<Self> {
        let data_dir = crate::utils::get_data_dir()?;
        let db_path = data_dir.join(crate::USER_DB_FILE);
        
        // Load existing database or create new one
        let database = if db_path.exists() {
            Self::load_database(&db_path)?
        } else {
            UserDatabase {
                users: HashMap::new(),
                version: 1,
            }
        };
        
        Ok(UserStorage {
            db_path,
            database,
        })
    }

    /// Load database from file
    fn load_database(path: &PathBuf) -> Result<UserDatabase> {
        let data = fs::read(path)?;
        
        // Try to deserialize with bincode first (more secure)
        if let Ok(db) = bincode::deserialize::<UserDatabase>(&data) {
            return Ok(db);
        }
        
        // Fallback to JSON if bincode fails (for compatibility)
        let json_str = String::from_utf8(data)?;
        let db: UserDatabase = serde_json::from_str(&json_str)?;
        Ok(db)
    }

    /// Save database to file
    fn save_database(&self) -> Result<()> {
        // Ensure directory exists
        if let Some(parent) = self.db_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Serialize to bincode for security
        let data = bincode::serialize(&self.database)?;
        
        // Write atomically (write to temp file then rename)
        let temp_path = self.db_path.with_extension("tmp");
        fs::write(&temp_path, data)?;
        fs::rename(temp_path, &self.db_path)?;
        
        Ok(())
    }

    /// Check if a user exists
    pub fn user_exists(&self, username: &str) -> Result<bool> {
        Ok(self.database.users.contains_key(username))
    }

    /// Save a user to storage
    pub fn save_user(&mut self, user: &User) -> Result<()> {
        self.database.users.insert(user.username.clone(), user.clone());
        self.save_database()?;
        Ok(())
    }

    /// Load a user from storage
    pub fn load_user(&self, username: &str) -> Result<Option<User>> {
        Ok(self.database.users.get(username).cloned())
    }

    /// Delete a user from storage
    pub fn delete_user(&mut self, username: &str) -> Result<()> {
        if self.database.users.remove(username).is_some() {
            self.save_database()?;
            Ok(())
        } else {
            anyhow::bail!("User '{}' not found", username)
        }
    }

    /// List all usernames
    pub fn list_usernames(&self) -> Result<Vec<String>> {
        let mut usernames: Vec<String> = self.database.users.keys().cloned().collect();
        usernames.sort();
        Ok(usernames)
    }

    /// Get total number of users
    pub fn user_count(&self) -> usize {
        self.database.users.len()
    }

    /// Clear all users (dangerous - use with caution)
    pub fn clear_all(&mut self) -> Result<()> {
        self.database.users.clear();
        self.save_database()?;
        Ok(())
    }

    /// Export database to JSON (for backup)
    pub fn export_json(&self, path: &PathBuf) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.database)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Import database from JSON (for restore)
    pub fn import_json(&mut self, path: &PathBuf) -> Result<()> {
        let json = fs::read_to_string(path)?;
        self.database = serde_json::from_str(&json)?;
        self.save_database()?;
        Ok(())
    }

    /// Get statistics about the user database
    pub fn get_stats(&self) -> UserStorageStats {
        let total_users = self.database.users.len();
        let active_users = self.database.users.values().filter(|u| u.is_active).count();
        let locked_users = self.database.users.values().filter(|u| u.is_locked()).count();
        let total_logins: u32 = self.database.users.values().map(|u| u.login_count).sum();
        let avg_reputation = if total_users > 0 {
            self.database.users.values().map(|u| u.reputation).sum::<i32>() / total_users as i32
        } else {
            0
        };

        UserStorageStats {
            total_users,
            active_users,
            locked_users,
            total_logins,
            avg_reputation,
        }
    }
}

/// Statistics about the user storage
#[derive(Debug, Clone)]
pub struct UserStorageStats {
    pub total_users: usize,
    pub active_users: usize,
    pub locked_users: usize,
    pub total_logins: u32,
    pub avg_reputation: i32,
}

impl Drop for UserStorage {
    /// Ensure database is saved when storage is dropped
    fn drop(&mut self) {
        // Attempt to save but ignore errors (can't propagate from Drop)
        let _ = self.save_database();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_storage() -> Result<UserStorage> {
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("test_users.db");
        
        Ok(UserStorage {
            db_path,
            database: UserDatabase::default(),
        })
    }

    #[test]
    fn test_user_storage_operations() -> Result<()> {
        let mut storage = create_test_storage()?;
        
        // Create test user
        let user = User::new("testuser".to_string(), "Password123!")?;
        
        // Test save
        storage.save_user(&user)?;
        assert!(storage.user_exists("testuser")?);
        
        // Test load
        let loaded = storage.load_user("testuser")?;
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().username, "testuser");
        
        // Test list
        let usernames = storage.list_usernames()?;
        assert_eq!(usernames.len(), 1);
        assert_eq!(usernames[0], "testuser");
        
        // Test delete
        storage.delete_user("testuser")?;
        assert!(!storage.user_exists("testuser")?);
        
        Ok(())
    }

    #[test]
    fn test_storage_stats() -> Result<()> {
        let mut storage = create_test_storage()?;
        
        // Add multiple users
        for i in 0..5 {
            let mut user = User::new(format!("user{}", i), "Password123!")?;
            if i == 3 {
                user.is_active = false;
            }
            user.reputation = i * 10;
            user.login_count = i as u32;
            storage.save_user(&user)?;
        }
        
        let stats = storage.get_stats();
        assert_eq!(stats.total_users, 5);
        assert_eq!(stats.active_users, 4);
        assert_eq!(stats.locked_users, 1);
        
        Ok(())
    }
}