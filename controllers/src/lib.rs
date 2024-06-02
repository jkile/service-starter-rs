use axum::Router;
use persistence::Db;

mod users_controller;

#[derive(Debug, Clone)]
pub struct AppState<T: Db> {
    pub db: T,
}

pub fn collect_routes<T: Db + 'static>() -> Router<AppState<T>> {
    let api_routes = Router::new().nest("/users", users_controller::collect_routes());
    api_routes
}
