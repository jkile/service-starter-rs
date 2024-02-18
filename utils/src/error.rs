use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

pub enum ApplicationError {
    ResourceNotFound(sqlx::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        // generates each response type based on which error is emitted
        match self {
            ApplicationError::ResourceNotFound(sql_error) => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    message: sql_error.to_string(),
                }),
            )
                .into_response(),
        }
    }
}
