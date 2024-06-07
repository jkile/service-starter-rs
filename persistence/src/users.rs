use axum_login::axum::async_trait;
use models::users::{User, UserId};
use utils::error::ApplicationError;

#[async_trait]
pub trait UsersTable {
    async fn get_user_by_id(&self, user_id: UserId) -> Result<User, ApplicationError>;
    async fn create_user(&self, user: User) -> Result<User, ApplicationError>;
    async fn check_username(&self, username: &String) -> Result<i64, ApplicationError>;
}
