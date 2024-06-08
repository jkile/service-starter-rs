use libsql::{Builder, Connection};

use crate::Db;

pub mod auth;
pub mod users;

#[derive(Clone)]
pub struct LibsqlDb {
    pub connection: Connection,
}

impl Db for LibsqlDb {}

impl LibsqlDb {
    #[cfg(any(release, devremote))]
    pub async fn new() -> LibsqlDb {
        let url = std::env::var("LIBSQL_URL").expect("LIBSQL_URL must be set");
        let token = std::env::var("LIBSQL_AUTH_TOKEN").unwrap_or_default();

        let db = Builder::new_remote_replica("local.db", url, token)
            .build()
            .await
            .unwrap();
        let connection = db.connect().unwrap();
        LibsqlDb { connection }
    }

    #[cfg(debug_assertions)]
    pub async fn new() -> LibsqlDb {
        if cfg!(test) {
            let db = Builder::new_local(":memory:").build().await.unwrap();
            let connection = db.connect().unwrap();
            LibsqlDb { connection }
        } else {
            let db = Builder::new_local("local.db").build().await.unwrap();
            let connection = db.connect().unwrap();
            let libsql_db = LibsqlDb { connection };
            libsql_db.migrate().await;
            libsql_db
        }
    }

    async fn migrate(&self) {
        self.connection
            .execute(
                "CREATE TABLE IF NOT EXISTS users (id TEXT PRIMARY KEY, username TEXT, password TEXT, access_token TEXT, permissions_type TEXT)",
                (),
            )
            .await
            .unwrap();
    }
}
