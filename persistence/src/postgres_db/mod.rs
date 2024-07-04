pub mod auth;
pub mod users;

use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use std::{ops::Deref, time::Duration};
use tracing::instrument;

use crate::Db;

impl Db for PostgresDb {
    // type Inner<'a> = Self where Self: 'a;
}

#[derive(Debug, Clone)]
pub struct PostgresDb(Pool<Postgres>);

impl Deref for PostgresDb {
    type Target = Pool<Postgres>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
        PostgresDb(pool)
    }

    pub async fn from_pool(pool: PgPool) -> PostgresDb {
        PostgresDb(pool)
    }
}
