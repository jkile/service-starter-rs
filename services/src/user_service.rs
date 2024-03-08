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
