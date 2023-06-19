use service_starter_rs::service_init;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    service_init().await
}
