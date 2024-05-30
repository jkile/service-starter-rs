use axum_login::{axum::async_trait, AuthnBackend};
use models::users::{Credentials, DbUser, User, UserId};
use password_auth::verify_password;
use thiserror::Error;
use tokio::task;
use utils::error::ApplicationError;

use crate::users::UsersTable;
use crate::PostgresDb;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
    #[error(transparent)]
    Sqlx(sqlx::Error),
    #[error(transparent)]
    AuthError(ApplicationError),
}

#[async_trait]
impl AuthnBackend for PostgresDb {
    type User = User;
    type Credentials = Credentials;
    type Error = BackendError;

    async fn authenticate(&self, credentials: Credentials) -> Result<Option<User>, Self::Error> {
        match credentials {
            Credentials::Password(password_cred) => {
                let user = sqlx::query_as::<_, DbUser>(
                    "SELECT * FROM users WHERE username = $1 AND password IS NOT NULL",
                )
                .bind(password_cred.username)
                .fetch_one(&self.conn_pool)
                .await
                .map_err(Self::Error::Sqlx);
                let converted_user: Option<User> = match user {
                    Ok(user) => Some(user.into()),
                    Err(err) => {
                        println!("{}", err);
                        return Err(err);
                    }
                };
                task::spawn_blocking(|| {
                    Ok(converted_user
                        .filter(|user| {
                            let Some(ref password) = user.password else {
                                return false;
                            };
                            verify_password(password_cred.password, password.as_str()).is_ok()
                        })
                        .into())
                })
                .await?
            }
        }
    }
    async fn get_user(&self, user_id: &UserId) -> Result<Option<User>, Self::Error> {
        Ok(Some(
            self.get_user_by_id(*user_id)
                .await
                .map_err(Self::Error::Sqlx)?,
        ))
    }
}
