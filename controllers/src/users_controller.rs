use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use models::users::{User, UserId};
use persistence::Db;
use services::user_service;
use utils::error::ApplicationError;

pub fn collect_routes() -> Router<Db> {
    Router::new().route("/:id", get(get_user))
}

async fn get_user(
    State(db): State<Db>,
    Path(id): Path<UserId>,
) -> Result<Json<User>, ApplicationError> {
    let user = user_service::get_user(db, id).await?;
    Ok(Json(user))
}
