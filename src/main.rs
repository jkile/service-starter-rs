use service_starter_rs::service_init;
use dotenv::dotenv;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv().ok();
    service_init().await
}
