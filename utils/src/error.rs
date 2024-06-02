use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Authentication error:  {0}")]
    AuthenticationError(String),
    #[error("Resource not found error: {0}")]
    ResourceNotFound(String),
    #[error("Failure creating resource error: {0}")]
    FailureCreatingResource(String),
    #[error("Unauthorized request")]
    UnauthorizedRequest(),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Resource conflict: {0}")]
    ResourceConflictError(String),
    #[error("Sql query failed: {0}")]
    SqlError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        // generates each response type based on which error is emitted
        match self {
            ApplicationError::AuthenticationError(message) => {
                (StatusCode::UNAUTHORIZED, Json(ErrorResponse { message })).into_response()
            }
            ApplicationError::ResourceNotFound(message) => {
                (StatusCode::NOT_FOUND, Json(ErrorResponse { message })).into_response()
            }
            ApplicationError::FailureCreatingResource(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message }),
            )
                .into_response(),
            ApplicationError::UnauthorizedRequest() => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: "Unauthorized request".to_string(),
                }),
            )
                .into_response(),
            ApplicationError::BadRequest(message) => {
                (StatusCode::BAD_REQUEST, Json(ErrorResponse { message })).into_response()
            }
            ApplicationError::ResourceConflictError(message) => {
                (StatusCode::CONFLICT, Json(ErrorResponse { message })).into_response()
            }
            ApplicationError::SqlError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { message }),
            )
                .into_response(),
        }
    }
}
