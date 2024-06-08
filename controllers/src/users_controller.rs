use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Form, Json, Router,
};
use axum_login::AuthSession;
use models::{
    permissions::{Permission, PermissionType},
    users::{Credentials, PasswordCredentials, User, UserExternal},
};
use persistence::Db;
use services::user_service;
use sqlx::types::Uuid;
use utils::error::ApplicationError;
use validator::Validate;

use crate::{require_login, AppState};

pub fn collect_routes<T: Db + 'static>() -> Router<AppState<T>> {
    Router::new()
        .route("/", get(get_user))
        .route("/logout", get(logout))
        .layer(middleware::from_fn(require_login::<T>))
        .route("/login", post(login))
        .route("/signup", post(signup))
}

async fn get_user<T: Db>(
    State(app_state): State<AppState<T>>,
    auth_session: AuthSession<T>,
) -> Result<Json<UserExternal>, ApplicationError> {
    let session_user = auth_session.user.unwrap();
    let user = user_service::get_user(&app_state.db, session_user.id).await?;
    Ok(Json(user.into()))
}

async fn login<T: Db>(
    State(_app_state): State<AppState<T>>,
    mut auth_session: AuthSession<T>,
    Form(credentials): Form<PasswordCredentials>,
) -> Result<Json<UserExternal>, ApplicationError> {
    credentials
        .validate()
        .map_err(|e| ApplicationError::ValidationError(e.to_string()))?;
    let user = match auth_session
        .authenticate(Credentials::Password(credentials))
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
    Ok(Json(user.into()))
}

async fn logout<T: Db>(
    State(_app_state): State<AppState<T>>,
    mut auth_session: AuthSession<T>,
) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn signup<T: Db>(
    State(app_state): State<AppState<T>>,
    mut auth_session: AuthSession<T>,
    Form(credentials): Form<PasswordCredentials>,
) -> Result<Json<UserExternal>, ApplicationError> {
    let user_id = Uuid::new_v4();
    let user_to_create = User::new(
        user_id,
        credentials.username.clone(),
        Some(credentials.password.clone()),
        None,
        Permission::from(PermissionType::User),
    );
    user_to_create
        .validate()
        .map_err(|e| ApplicationError::ValidationError(e.to_string()))?;
    let user = user_service::create_user(&app_state.db, user_to_create).await;
    if let Err(err) = user {
        return Err(err);
    }
    let authed_user = match auth_session
        .authenticate(Credentials::Password(credentials))
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
    Ok(Json(user.unwrap().into()))
}
