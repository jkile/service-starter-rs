use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use models::users::{User, UserId};
use persistence::Db;
use serde::Deserialize;
use services::user_service;
use sqlx::types::Uuid;
use utils::error::ApplicationError;

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    password: String,
}

pub fn collect_routes() -> Router<Db> {
    Router::new()
        .route("/:id", get(get_user))
        .route("/", post(create_user))
}

async fn get_user(
    State(db): State<Db>,
    Path(id): Path<UserId>,
) -> Result<Json<User>, ApplicationError> {
    let user = user_service::get_user(db, id).await?;
    Ok(Json(user))
}

async fn create_user(
    State(db): State<Db>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, ApplicationError> {
    let user_id = Uuid::new_v4();
    let user_to_create = User::new(user_id, payload.username, payload.password);
    let user = user_service::create_user(db, user_to_create).await?;
    Ok(Json(user))
}
