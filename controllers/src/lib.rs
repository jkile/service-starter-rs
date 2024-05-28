use axum::Router;
use persistence::PostgresDb;

mod users_controller;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: PostgresDb,
}

pub fn collect_routes() -> Router<AppState> {
    let api_routes = Router::new().nest("/users", users_controller::collect_routes());
    api_routes
}
