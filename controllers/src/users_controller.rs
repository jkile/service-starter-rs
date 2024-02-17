use axum::{http::StatusCode, routing::get, Json, Router};
use models::users::User;
use tracing::{info, instrument};

pub struct UsersController {}

impl UsersController {
    pub fn collect_routes() -> Router {
        Router::new().route("/:id", get(Self::get_user))
    }

    #[instrument(level = "info")]
    async fn get_user() -> (StatusCode, Json<User>) {
        let id = String::from("asdf");
        let username = String::from("asdf");
        let password = String::from("asdf");
        let dummy_user = User::new(id, username, password);
        (StatusCode::OK, Json(dummy_user))
    }
}
