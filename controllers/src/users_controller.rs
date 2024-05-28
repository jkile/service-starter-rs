use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Form, Json, Router,
};
use axum_login::{login_required, AuthSession};
use models::users::{Credentials, PasswordCredentials, User, UserId};
use persistence::PostgresDb;
use serde::Deserialize;
use services::user_service;
use sqlx::types::Uuid;
//use tower_http::auth::AsyncRequireAuthorizationLayer;
use utils::error::ApplicationError;

use crate::AppState;

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    password: String,
}

pub fn collect_routes() -> Router<AppState> {
    Router::new()
        .route("/:id", get(get_user))
        .route_layer(login_required!(PostgresDb))
        .route("/login", post(login))
        .route("/logout", get(logout))
        .route("/signup", post(signup))
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
    let user_to_create = User::new(user_id, payload.username, Some(payload.password), None);
    let user = user_service::create_user(app_state.db.clone(), user_to_create).await?;
    Ok(Json(user))
}

async fn login(
    State(_app_state): State<AppState>,
    mut auth_session: AuthSession<PostgresDb>,
    Form(credentials): Form<PasswordCredentials>,
) -> Result<Json<User>, ApplicationError> {
    let user = match auth_session
        .authenticate(Credentials::Password(credentials.clone()))
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => return Err(ApplicationError::UnauthorizedRequest()),
        Err(_) => {
            return Err(ApplicationError::AuthenticationError(
                "Failed authentication".to_string(),
            ))
        }
    };

    if auth_session.login(&user).await.is_err() {
        return Err(ApplicationError::AuthenticationError(
            "Failed authentication".to_string(),
        ));
    }
    Ok(Json(user))
}

async fn logout(
    State(_app_state): State<AppState>,
    mut auth_session: AuthSession<PostgresDb>,
) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn signup(
    State(app_state): State<AppState>,
    mut auth_session: AuthSession<PostgresDb>,
    Form(credentials): Form<PasswordCredentials>,
) -> Result<Json<User>, ApplicationError> {
    let user_id = Uuid::new_v4();
    let user_to_create = User::new(
        user_id,
        credentials.username.clone(),
        Some(credentials.password.clone()),
        None,
    );
    let user = user_service::create_user(app_state.db, user_to_create).await;
    if let Err(err) = user {
        return Err(err);
    }
    let authed_user = match auth_session
        .authenticate(Credentials::Password(credentials.clone()))
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => return Err(ApplicationError::UnauthorizedRequest()),
        Err(_) => {
            return Err(ApplicationError::AuthenticationError(
                "Failed authentication".to_string(),
            ))
        }
    };

    if auth_session.login(&authed_user).await.is_err() {
        return Err(ApplicationError::AuthenticationError(
            "Failed authentication".to_string(),
        ));
    }

    Ok(Json(user.unwrap()))
    // redirect logic
    // if let Some(ref next) = credentials.next {
    //     Redirect::to(next)
    // } else {
    //     Redirect::to("/")
    // }
    // .into_response()
}
