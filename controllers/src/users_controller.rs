use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use models::users::{User, UserId};
use persistence::Db;
use services::user_service::UserService;
use tracing::instrument;
use utils::error::ApplicationError;

pub struct UsersController {}

impl UsersController {
    pub fn collect_routes() -> Router<Db> {
        Router::new().route("/:id", get(Self::get_user))
    }

    #[instrument(level = "info")]
    async fn get_user(
        State(db): State<Db>,
        Path(id): Path<UserId>,
    ) -> Result<Json<User>, ApplicationError> {
        let user = UserService::get_user(db, id).await?;
        Ok(Json(user))
    }
}
