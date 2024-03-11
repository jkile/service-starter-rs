use models::users::{SessionToken, User, UserId};
use persistence::{sessions::SessionsTable, users::UsersTable, Db};
use tracing::error;
use utils::error::ApplicationError;

pub async fn get_user(db: Db, user_id: UserId) -> Result<User, ApplicationError> {
    let user = db.get_user(user_id).await;
    match user {
        Ok(user) => Ok(user),
        Err(e) => {
            error!("failed to retrieve user from table: {}", e);
            Err(ApplicationError::ResourceNotFound(e))
        }
    }
}

pub async fn create_user(db: Db, user: User) -> Result<User, ApplicationError> {
    let created_user = db.create_user(user).await;
    match created_user {
        Ok(user) => Ok(user),
        Err(e) => {
            error!("failed to create entry in users table: {}", e);
            Err(ApplicationError::FailureCreatingResource(e))
        }
    }
}

pub async fn create_session(db: Db, user_id: UserId) -> Result<SessionToken, ApplicationError> {
    let session = db.new_session("test".to_string(), user_id).await;
    match session {
        Ok(session) => Ok(session),
        Err(e) => {
            error!("failed to create entry in sessions table: {}", e);
            Err(ApplicationError::FailureCreatingResource(e))
        }
    }
}
