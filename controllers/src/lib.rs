use axum::Router;
use persistence::Db;

mod users_controller;

pub fn collect_routes() -> Router<Db> {
    let api_routes = Router::new().nest("/users", users_controller::collect_routes());
    api_routes
}
