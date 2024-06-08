use axum_login::axum::async_trait;
use models::permissions::PermissionsType;
use models::users::{DbUser, User, UserId};
use utils::error::ApplicationError;

use crate::users::UsersTable;

use super::PostgresDb;

#[async_trait]
impl UsersTable for PostgresDb {
    async fn get_user_by_id(&self, user_id: UserId) -> Result<User, ApplicationError> {
        let row = sqlx::query_as!(
            DbUser,
            r#"SELECT id, username, password, access_token,
            permissions_type AS "permissions_type: PermissionsType"
            FROM users WHERE users.id = $1"#,
            user_id
        )
        .fetch_one(&self.conn_pool)
        .await
        .map_err(|e| ApplicationError::SqlError(e.to_string()))?;
        Ok(row.into())
    }

    async fn create_user(&self, user: User) -> Result<User, ApplicationError> {
        let hashed_password = password_auth::generate_hash(user.password.unwrap().as_str());
        let user = sqlx::query_as!(
            DbUser,
            r#"INSERT INTO users (id, username, password, access_token, permissions_type)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, username, password, access_token,
            permissions_type AS "permissions_type: PermissionsType""#,
            user.id,
            user.username,
            hashed_password,
            user.access_token,
            user.permissions.permissions_type as PermissionsType
        )
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
