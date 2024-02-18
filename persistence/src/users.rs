use models::users::{User, UserId};
use tracing::instrument;

use crate::Db;

#[allow(async_fn_in_trait)]
pub trait UsersTable {
    async fn get_user(&self, user_id: UserId) -> Result<User, sqlx::Error>;
}

impl UsersTable for Db {
    #[instrument]
    async fn get_user(&self, user_id: UserId) -> Result<User, sqlx::Error> {
        let row = sqlx::query_as::<_, User>("SELECT * FROM users WHERE users.id = $1")
            .bind(user_id)
            .fetch_one(&self.conn_pool)
            .await;
        row
    }
}
