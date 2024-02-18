use axum::Router;
use persistence::Db;
use users_controller::UsersController;

mod users_controller;

pub struct Controllers {}

impl Controllers {
    pub fn new() -> Controllers {
        Controllers {}
    }

    pub fn collect_routes() -> Router<Db> {
        let api_routes = Router::new().nest("/users", UsersController::collect_routes());
        api_routes
    }
}
