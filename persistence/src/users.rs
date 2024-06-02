use axum_login::axum::async_trait;
use models::users::{DbUser, User, UserId};
use utils::error::ApplicationError;

use crate::PostgresDb;

#[async_trait]
pub trait UsersTable {
    async fn get_user_by_id(&self, user_id: UserId) -> Result<User, ApplicationError>;
    async fn create_user(&self, user: User) -> Result<User, ApplicationError>;
    async fn check_username(&self, username: &String) -> Result<i64, ApplicationError>;
}

#[async_trait]
impl UsersTable for PostgresDb {
    async fn get_user_by_id(&self, user_id: UserId) -> Result<User, ApplicationError> {
        let row = sqlx::query_as::<_, DbUser>("SELECT * FROM users WHERE users.id = $1")
            .bind(user_id)
            .fetch_one(&self.conn_pool)
            .await
            .map_err(|e| ApplicationError::SqlError(e.to_string()))?;
        Ok(row.into())
    }

    async fn create_user(&self, user: User) -> Result<User, ApplicationError> {
        let hashed_password = password_auth::generate_hash(user.password.unwrap().as_str());
        let user = sqlx::query_as::<_, DbUser>(
            "INSERT INTO users (id, username, password, access_token, permissions)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, username, password, access_token, permissions",
        )
        .bind(user.id)
        .bind(user.username)
        .bind(hashed_password)
        .bind(user.access_token)
        .bind(user.permissions.permission_type)
        .fetch_one(&self.conn_pool)
        .await
        .map_err(|e| ApplicationError::SqlError(e.to_string()))?;
        Ok(user.into())
    }

    async fn check_username(&self, username: &String) -> Result<i64, ApplicationError> {
        let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(&self.conn_pool)
            .await
            .map_err(|e| ApplicationError::SqlError(e.to_string()))?;
        Ok(user_count)
    }
}
