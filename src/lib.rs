use actix_web::{App, HttpServer};
use actix_web::web::Data;
use diesel::{prelude::*, r2d2};
use std::{env};
use dotenv::dotenv;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

struct AppState {
    pub connection: DbPool
}

pub async fn service_init() -> std::io::Result<()> {
    dotenv().ok();
    let db_pool = build_db_pool();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                connection: db_pool.clone()
            }))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await

}

fn build_db_pool() -> DbPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Db URL not valid")
}
