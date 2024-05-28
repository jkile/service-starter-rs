use axum_login::{axum::async_trait, AuthnBackend};
use models::users::{Credentials, User, UserId};
use password_auth::verify_password;
use thiserror::Error;
use tokio::task;
use utils::error::ApplicationError;

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
                let user = sqlx::query_as::<_, User>(
                    "SELECT * FROM users WHERE username = $1 AND password IS NOT NULL",
                )
                .bind(password_cred.username)
                .fetch_optional(&self.conn_pool)
                .await
                .map_err(Self::Error::Sqlx)?;

                task::spawn_blocking(|| {
                    Ok(user.filter(|user| {
                        let Some(ref password) = user.password else {
                            return false;
                        };
                        verify_password(password_cred.password, password.as_str()).is_ok()
                    }))
                })
                .await?
            }
        }
    }
    async fn get_user(&self, user_id: &UserId) -> Result<Option<User>, Self::Error> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE users.id = $1")
                .bind(user_id)
                .fetch_optional(&self.conn_pool)
                .await
                .map_err(Self::Error::Sqlx)?,
        )
    }
}
