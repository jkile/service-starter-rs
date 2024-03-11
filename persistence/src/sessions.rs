use models::users::{SessionToken, UserId};

use crate::Db;

#[allow(async_fn_in_trait)]
pub trait SessionsTable {
    async fn new_session(
        &self,
        session_token: String,
        user_id: UserId,
    ) -> Result<SessionToken, sqlx::Error>;
}

impl SessionsTable for Db {
    async fn new_session(
        &self,
        session_token: String,
        user_id: UserId,
    ) -> Result<SessionToken, sqlx::Error> {
        let token = sqlx::query_as::<_, SessionToken>(
            "INSERT INTO sessions (session_token, user_id) VALUES ($1, $2)
            RETURNING session_token, user_id;",
        )
        .bind(session_token)
        .bind(user_id)
        .fetch_one(&self.conn_pool)
        .await;
        token
    }
}
