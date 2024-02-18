use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

pub mod users;

#[derive(Debug, Clone)]
pub struct Db {
    pub conn_pool: Pool<Postgres>,
}

impl Db {
    pub async fn new() -> Db {
        let db_connection_str = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432".to_string());
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&db_connection_str)
            .await
            .expect("connection to database failed");
        Db { conn_pool: pool }
    }
}
