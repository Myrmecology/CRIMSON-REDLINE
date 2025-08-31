//! Authentication system for CRIMSON-REDLINE

pub mod login;
pub mod register;
pub mod storage;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use anyhow::Result;

/// User structure representing an agent in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub login_count: u32,
    pub reputation: i32,
    pub is_active: bool,
    pub failed_attempts: u32,
}

impl User {
    /// Create a new user with hashed password
    pub fn new(username: String, password: &str) -> Result<Self> {
        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
        
        Ok(User {
            username,
            password_hash,
            created_at: Utc::now(),
            last_login: None,
            login_count: 0,
            reputation: 0,
            is_active: true,
            failed_attempts: 0,
        })
    }

    /// Verify password against hash
    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password_hash).unwrap_or(false)
    }

    /// Update last login timestamp
    pub fn update_login(&mut self) {
        self.last_login = Some(Utc::now());
        self.login_count += 1;
        self.failed_attempts = 0; // Reset failed attempts on successful login
    }

    /// Record a failed login attempt
    pub fn record_failed_attempt(&mut self) {
        self.failed_attempts += 1;
        if self.failed_attempts >= 5 {
            self.is_active = false; // Lock account after 5 failed attempts
        }
    }

    /// Check if account is locked
    pub fn is_locked(&self) -> bool {
        !self.is_active || self.failed_attempts >= 5
    }

    /// Unlock the account
    pub fn unlock(&mut self) {
        self.is_active = true;
        self.failed_attempts = 0;
    }
}

/// Main authentication system
pub struct AuthSystem {
    storage: storage::UserStorage,
    current_user: Option<User>,
    config: crate::utils::Config,
}

impl AuthSystem {
    /// Create a new authentication system
    pub fn new() -> Result<Self> {
        let storage = storage::UserStorage::new()?;
        let config = crate::utils::Config::load()?;
        
        Ok(AuthSystem {
            storage,
            current_user: None,
            config,
        })
    }

    /// Register a new user
    pub async fn register(&mut self, username: &str, password: &str, confirm_password: &str) -> Result<User> {
        // Validate passwords match
        if password != confirm_password {
            anyhow::bail!("Passwords do not match");
        }

        // Validate password strength
        self.config.validate_password(password)?;

        // Check if username already exists
        if self.storage.user_exists(username)? {
            anyhow::bail!("Username '{}' already exists", username);
        }

        // Validate username
        if username.len() < 3 {
            anyhow::bail!("Username must be at least 3 characters long");
        }

        if username.len() > 20 {
            anyhow::bail!("Username must be 20 characters or less");
        }

        if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            anyhow::bail!("Username can only contain letters, numbers, and underscores");
        }

        // Create new user
        let user = User::new(username.to_string(), password)?;
        
        // Save to storage
        self.storage.save_user(&user)?;
        
        Ok(user)
    }

    /// Login an existing user
    pub async fn login(&mut self, username: &str, password: &str) -> Result<User> {
        // Load user from storage
        let mut user = self.storage.load_user(username)?
            .ok_or_else(|| anyhow::anyhow!("Invalid username or password"))?;

        // Check if account is locked
        if user.is_locked() {
            anyhow::bail!("Account is locked due to multiple failed login attempts");
        }

        // Verify password
        if !user.verify_password(password) {
            user.record_failed_attempt();
            self.storage.save_user(&user)?;
            anyhow::bail!("Invalid username or password");
        }

        // Update login info
        user.update_login();
        self.storage.save_user(&user)?;
        
        // Set current user
        self.current_user = Some(user.clone());
        
        Ok(user)
    }

    /// Logout current user
    pub fn logout(&mut self) {
        self.current_user = None;
    }

    /// Get current logged-in user
    pub fn current_user(&self) -> Option<&User> {
        self.current_user.as_ref()
    }

    /// Check if a user is logged in
    pub fn is_authenticated(&self) -> bool {
        self.current_user.is_some()
    }

    /// Get all registered usernames (for display purposes)
    pub fn list_users(&self) -> Result<Vec<String>> {
        self.storage.list_usernames()
    }

    /// Update current user's reputation
    pub fn update_reputation(&mut self, change: i32) -> Result<()> {
        if let Some(ref mut user) = self.current_user {
            user.reputation += change;
            self.storage.save_user(user)?;
        }
        Ok(())
    }

    /// Delete a user (admin function)
    pub fn delete_user(&mut self, username: &str) -> Result<()> {
        // Cannot delete current user
        if let Some(ref current) = self.current_user {
            if current.username == username {
                anyhow::bail!("Cannot delete currently logged-in user");
            }
        }
        
        self.storage.delete_user(username)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new("testuser".to_string(), "Password123!").unwrap();
        assert_eq!(user.username, "testuser");
        assert_eq!(user.login_count, 0);
        assert_eq!(user.reputation, 0);
        assert!(user.is_active);
    }

    #[test]
    fn test_password_verification() {
        let user = User::new("testuser".to_string(), "Password123!").unwrap();
        assert!(user.verify_password("Password123!"));
        assert!(!user.verify_password("wrongpassword"));
    }

    #[test]
    fn test_failed_attempts() {
        let mut user = User::new("testuser".to_string(), "Password123!").unwrap();
        
        for _ in 0..4 {
            user.record_failed_attempt();
            assert!(!user.is_locked());
        }
        
        user.record_failed_attempt();
        assert!(user.is_locked());
        
        user.unlock();
        assert!(!user.is_locked());
    }
}