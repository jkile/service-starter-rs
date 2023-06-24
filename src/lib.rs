use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::{prelude::*, r2d2};
use std::env;
use tracing;
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

struct AppState {
    pub connection: DbPool,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

pub async fn service_init() -> std::io::Result<()> {
    init_telemetry();
    let db_pool = build_db_pool();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                connection: db_pool.clone(),
            }))
            .wrap(TracingLogger::default())
            .service(hello)
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

fn init_telemetry() {
    let app_name = "service-starter-rs";

    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));

    let formatting_layer = BunyanFormattingLayer::new(app_name.into(), std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to install 'tracing' subscriber")
}
