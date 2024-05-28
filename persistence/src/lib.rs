use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use tower_sessions_sqlx_store::PostgresStore;
use tracing::instrument;

pub mod auth;
pub mod users;

#[derive(Debug, Clone)]
pub struct PostgresDb {
    pub conn_pool: Pool<Postgres>,
    pub session_store: PostgresStore,
}

impl PostgresDb {
    #[instrument]
    pub async fn new() -> PostgresDb {
        let db_connection_str = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://jakekile:password@localhost:5432/jakekile".to_string()
        });
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&db_connection_str)
            .await
            .expect("connection to database failed");

        let session_pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&db_connection_str)
            .await
            .expect("session pool connection to database failed");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("SQL migration failed");

        let session_store = PostgresStore::new(session_pool);
        session_store.migrate().await.unwrap();

        PostgresDb {
            conn_pool: pool,
            session_store,
        }
    }
}
