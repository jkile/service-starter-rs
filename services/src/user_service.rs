use models::users::{User, UserId};
use persistence::{users::UsersTable, Db};
use tracing::{error, instrument};
use utils::error::ApplicationError;

pub struct UserService {}

impl UserService {
    #[instrument(level = "info")]
    pub async fn get_user(db: Db, user_id: UserId) -> Result<User, ApplicationError> {
        let user = db.get_user(user_id).await;
        match user {
            Ok(user) => Ok(user),
            Err(e) => {
                error!("failed to retrieve user from table: ");
                Err(ApplicationError::ResourceNotFound(e))
            }
        }
    }
}
