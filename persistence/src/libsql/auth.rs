use axum_login::{axum::async_trait, AuthnBackend};
use libsql::de;
use models::users::{Credentials, DbUser, User, UserId};
use password_auth::verify_password;
use thiserror::Error;
use tokio::task;
use utils::error::ApplicationError;

use super::LibsqlDb;

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
impl AuthnBackend for LibsqlDb {
    type User = User;
    type Credentials = Credentials;
    type Error = BackendError;

    async fn authenticate(&self, credentials: Credentials) -> Result<Option<User>, Self::Error> {
        match credentials {
            Credentials::Password(password_cred) => {
                let mut statement = self
                    .connection
                    .prepare("SELECT * FROM users WHERE username = ?1 AND password IS NOT NULL")
                    .await
                    .unwrap();
                let row = statement
                    .query([password_cred.username])
                    .await
                    .unwrap()
                    .next()
                    .await
                    .unwrap()
                    .unwrap();
                let user = de::from_row::<DbUser>(&row);
                let converted_user: Option<User> = match user {
                    Ok(user) => Some(user.into()),
                    Err(err) => {
                        println!("{}", err);
                        return Err(BackendError::AuthError(ApplicationError::SqlError(
                            err.to_string(),
                        )));
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
        let mut statement = self
            .connection
            .prepare("SELECT * FROM users WHERE id = ?1")
            .await
            .unwrap();
        let row = statement
            .query([user_id.to_string()])
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
            .unwrap();
        let user = de::from_row::<DbUser>(&row);
        if let Ok(user) = user {
            Ok(Some(user.into()))
        } else {
            Err(Self::Error::AuthError(ApplicationError::SqlError(
                "Failed to get user".to_string(),
            )))
        }
    }
}
