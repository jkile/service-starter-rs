use thiserror::Error;
use tokio::task;
use utils::error::ApplicationError;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
    #[error(transparent)]
    Sqlx(sqlx::Error),
    #[error(transparent)]
    AuthError(ApplicationError),
}
