use models::users::{User, UserId};
use persistence::Db;
use tracing::error;
use utils::error::ApplicationError;

pub async fn get_user<T: Db>(db: &T, user_id: UserId) -> Result<User, ApplicationError> {
    let user = db.get_user_by_id(user_id).await;
    match user {
        Ok(user) => Ok(user),
        Err(e) => {
            error!("failed to retrieve user from table: {}", e);
            Err(ApplicationError::ResourceNotFound(
                "Failed to retrieve user".to_string(),
            ))
        }
    }
}

pub async fn create_user<T: Db>(db: &T, user: User) -> Result<User, ApplicationError> {
    if let None = user.password {
        return Err(ApplicationError::BadRequest(
            "No password included with user".to_string(),
        ));
    }
    check_unique_username(db, &user).await?;
    let created_user = db.create_user(user).await;
    match created_user {
        Ok(user) => Ok(user),
        Err(e) => {
            error!("failed to create entry in users table: {}", e);
            Err(ApplicationError::FailureCreatingResource(
                "
                Failed to create new user"
                    .to_string(),
            ))
        }
    }
}

async fn check_unique_username<T: Db>(db: &T, user: &User) -> Result<(), ApplicationError> {
    match db.check_username(&user.username).await {
        Ok(count) => {
            if count == 0 {
                return Ok(());
            } else {
                return Err(ApplicationError::ResourceConflictError(
                    "Username already exists".to_string(),
                ));
            }
        }
        Err(err) => {
            error!("failed query to check if username exists: {}", err);
            Err(ApplicationError::ResourceNotFound(
                "Failed to check user database".to_string(),
            ))
        }
    }
}
