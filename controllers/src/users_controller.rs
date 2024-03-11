use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use models::users::{User, UserId};
use serde::Deserialize;
use services::user_service;
use sqlx::types::Uuid;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use utils::error::ApplicationError;

use crate::AppState;

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

pub fn collect_routes() -> Router<AppState> {
    Router::new()
        .route("/:id", get(get_user))
        // All unprotected routes live beneath this layer.
        .layer(AsyncRequireAuthorizationLayer::new(
            utils::auth_layer::ClerkAuth,
        ))
        .route("/", post(create_user))
}

async fn get_user(
    State(app_state): State<AppState>,
    Path(id): Path<UserId>,
) -> Result<Json<User>, ApplicationError> {
    let user = user_service::get_user(app_state.db, id).await?;
    Ok(Json(user))
}

async fn create_user(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, ApplicationError> {
    let user_id = Uuid::new_v4();
    let user_to_create = User::new(user_id, payload.username);
    let user = user_service::create_user(app_state.db.clone(), user_to_create).await?;
    Ok(Json(user))
}
