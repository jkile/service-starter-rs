use models::users::{User, UserId};
use tracing::instrument;

use crate::PostgresDb;

#[allow(async_fn_in_trait)]
pub trait UsersTable {
    async fn get_user(&self, user_id: UserId) -> Result<User, sqlx::Error>;
    async fn create_user(&self, user: User) -> Result<User, sqlx::Error>;
    async fn check_username(&self, username: &String) -> Result<i64, sqlx::Error>;
}

impl UsersTable for PostgresDb {
    #[instrument]
    async fn get_user(&self, user_id: UserId) -> Result<User, sqlx::Error> {
        let row = sqlx::query_as::<_, User>("SELECT * FROM users WHERE users.id = $1")
            .bind(user_id)
            .fetch_one(&self.conn_pool)
            .await;
        row
    }

    #[instrument]
    async fn create_user(&self, user: User) -> Result<User, sqlx::Error> {
        let hashed_password = password_auth::generate_hash(user.password.unwrap().as_str());
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, username, password, access_token)
            VALUES ($1, $2, $3, $4)
            RETURNING id, username, password, access_token",
        )
        .bind(user.id)
        .bind(user.username)
        .bind(hashed_password)
        .bind(user.access_token)
        .fetch_one(&self.conn_pool)
        .await;
        user
    }

    #[instrument]
    async fn check_username(&self, username: &String) -> Result<i64, sqlx::Error> {
        let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(&self.conn_pool)
            .await?;
        Ok(user_count)
    }
}
