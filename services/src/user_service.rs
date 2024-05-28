use models::users::{User, UserId};
use persistence::{users::UsersTable, PostgresDb};
use tracing::error;
use utils::error::ApplicationError;

pub async fn get_user(db: &PostgresDb, user_id: UserId) -> Result<User, ApplicationError> {
    let user = db.get_user(user_id).await;
    match user {
        Ok(user) => Ok(user),
        Err(e) => {
            error!("failed to retrieve user from table: {}", e);
            Err(ApplicationError::ResourceNotFound(e))
        }
    }
}

pub async fn create_user(db: &PostgresDb, user: User) -> Result<User, ApplicationError> {
    if let None = user.password {
        return Err(ApplicationError::BadRequest(
            "No password included with user".to_string(),
        ));
    }
    check_unique_username(&db, &user).await?;
    let created_user = db.create_user(user).await;
    match created_user {
        Ok(user) => Ok(user),
        Err(e) => {
            error!("failed to create entry in users table: {}", e);
            Err(ApplicationError::FailureCreatingResource(e))
        }
    }
}

async fn check_unique_username(db: &PostgresDb, user: &User) -> Result<(), ApplicationError> {
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
        Err(err) => Err(ApplicationError::ResourceNotFound(err)),
    }
}
