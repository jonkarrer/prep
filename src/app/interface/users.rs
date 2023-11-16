use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::domain::entity::User;

#[async_trait::async_trait]
pub trait UserRepository: Sync + Send {
    async fn create_user(&self, email: &str, credentials_id: &str) -> Result<String>;
    async fn get_user_by_email(&self, email: &str) -> Result<User>;
    async fn get_user_by_id(&self, user_id: &str) -> Result<User>;
    async fn insert_reset_password_details(
        &self,
        reset_token: &str,
        expiration: &DateTime<Utc>,
        user_id: &str,
    ) -> Result<()>;
}
