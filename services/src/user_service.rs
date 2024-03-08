use models::users::{User, UserId};
use persistence::{users::UsersTable, Db};
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
            error!("failed to create entry is users table: {}", e);
            Err(ApplicationError::FailureCreatingResource(e))
        }
    }
}
