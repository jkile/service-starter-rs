use axum_login::{axum::async_trait, AuthnBackend};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use tower_sessions_core::SessionStore;
use tower_sessions_sqlx_store::PostgresStore;
use tracing::instrument;
use users::UsersTable;

pub mod auth;
pub mod users;

#[async_trait]
pub trait Db: UsersTable + AuthnBackend + Clone + Send + Sync {
    fn get_session_store(&self) -> impl SessionStore + Clone;
}

impl Db for PostgresDb {
    fn get_session_store(&self) -> impl SessionStore + Clone {
        self.session_store.clone()
    }
}

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
