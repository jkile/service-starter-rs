use axum::Router;
use persistence::Db;

mod users_controller;
mod webhooks_controller;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Db,
}

pub fn collect_routes() -> Router<AppState> {
    let api_routes = Router::new()
        .nest("/users", users_controller::collect_routes())
        .nest("/webhooks", webhooks_controller::collect_routes());
    api_routes
}
