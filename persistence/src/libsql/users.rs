use axum_login::axum::async_trait;
use libsql::de;
use models::users::{DbUser, User, UserId};
use utils::error::ApplicationError;

use crate::users::UsersTable;

use super::LibsqlDb;

#[async_trait]
impl UsersTable for LibsqlDb {
    async fn get_user_by_id(&self, user_id: UserId) -> Result<User, ApplicationError> {
        let mut statement = self
            .connection
            .prepare("SELECT * FROM users WHERE users.id = ?1")
            .await
            .map_err(|e| ApplicationError::SqlError(e.to_string()))?;
        let row = statement
            .query([user_id.to_string()])
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
            .unwrap();
        let user =
            de::from_row::<DbUser>(&row).map_err(|e| ApplicationError::SqlError(e.to_string()))?;
        Ok(user.into())
    }

    async fn create_user(&self, user: User) -> Result<User, ApplicationError> {
        let hashed_password = password_auth::generate_hash(user.password.unwrap().as_str());
        let access_token = if let Some(token) = user.access_token {
            token
        } else {
            "".to_string()
        };
        let mut statement = self.connection.prepare("INSERT INTO users (id, username, password, access_token, permissions_type)
            VALUES (?1, ?2, ?3, ?4, ?5) RETURNING id, username, password, access_token, permissions_type")
        .await
        .map_err(|e| ApplicationError::SqlError(e.to_string()))?;
        let row = statement
            .query([
                user.id.to_string(),
                user.username,
                hashed_password,
                access_token,
                user.permissions.permissions_type.to_string().to_lowercase(),
            ])
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
            .unwrap();
        let user =
            de::from_row::<DbUser>(&row).map_err(|e| ApplicationError::SqlError(e.to_string()))?;
        Ok(user.into())
    }

    async fn check_username(&self, username: &String) -> Result<i64, ApplicationError> {
        let mut statement = self
            .connection
            .prepare("SELECT id FROM users WHERE username = ?1")
            .await
            .map_err(|e| ApplicationError::SqlError(e.to_string()))?;
        let mut rows = statement.query([username.clone()]).await.unwrap();
        let mut user_count = 0;
        while let Ok(Some(_)) = rows.next().await {
            user_count += 1;
        }
        // let user_count =
        //     de::from_row::<String>(&row).map_err(|e| ApplicationError::SqlError(e.to_string()))?;
        Ok(user_count as i64)
    }
}
