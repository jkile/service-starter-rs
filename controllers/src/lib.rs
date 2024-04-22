use axum::Router;
use persistence::Db;

mod templates_controller;
mod users_controller;
mod webhooks_controller;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Db,
}

pub fn collect_api_routes() -> Router<AppState> {
    let api_routes = Router::new()
        .nest("/users", users_controller::collect_routes())
        .nest("/webhooks", webhooks_controller::collect_routes());
    api_routes
}
pub fn collect_template_routes() -> Router<AppState> {
    let template_routes = Router::new().nest("/", templates_controller::collect_routes());
    template_routes
}

pub fn collect_fallback_route() -> Router<AppState> {
    Router::new().fallback(templates_controller::handle_404)
}
