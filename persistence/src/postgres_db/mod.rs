pub mod auth;
pub mod users;

use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use std::time::Duration;
use tracing::instrument;

use crate::Db;

impl Db for PostgresDb {}

#[derive(Debug, Clone)]
pub struct PostgresDb {
    pub conn_pool: Pool<Postgres>,
}

impl PostgresDb {
    #[instrument]
    pub async fn new() -> PostgresDb {
        let db_connection_str = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://jakekile:password@localhost:5432/jakekile".to_string()
        });
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&db_connection_str)
            .await
            .expect("connection to database failed");
        sqlx::migrate!().run(&pool).await.unwrap();
        PostgresDb { conn_pool: pool }
    }

    pub async fn from_pool(pool: PgPool) -> PostgresDb {
        PostgresDb { conn_pool: pool }
    }
}
