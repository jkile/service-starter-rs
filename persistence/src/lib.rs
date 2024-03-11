use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use tracing::instrument;

pub mod sessions;
pub mod users;

#[derive(Debug, Clone)]
pub struct Db {
    pub conn_pool: Pool<Postgres>,
}

impl Db {
    #[instrument]
    pub async fn new() -> Db {
        let db_connection_str = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://jakekile:password@localhost:5432/jakekile".to_string()
        });
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&db_connection_str)
            .await
            .expect("connection to database failed");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("SQL migration failed");
        Db { conn_pool: pool }
    }
}
