use axum_login::{axum::async_trait, AuthnBackend};
use models::users::{Credentials, User};
use users::UsersTable;

pub mod auth;
pub mod in_memory_db;
pub mod postgres_db;
pub mod users;

#[async_trait]
pub trait Db:
    UsersTable + AuthnBackend<User = User, Credentials = Credentials> + Send + Sync
{
}
