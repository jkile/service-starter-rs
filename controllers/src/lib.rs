use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    Router,
};
use axum_login::AuthSession;
use persistence::Db;
use utils::error::ApplicationError;

mod users_controller;

#[derive(Debug, Clone)]
pub struct AppState<T: Db> {
    pub db: T,
}

pub fn collect_routes<T: Db + 'static>() -> Router<AppState<T>> {
    let api_routes = Router::new().nest("/users", users_controller::collect_routes());
    api_routes
}

pub async fn require_login<T: Db>(
    auth_session: AuthSession<T>,
    request: Request,
    next: Next,
) -> Response {
    if auth_session.user.is_some() {
        return next.run(request).await;
    } else {
        return ApplicationError::UnauthorizedRequest().into_response();
    }
}
