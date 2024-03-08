use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: UserId,
    pub username: Username,
    pub password: UserPassword,
}

pub type UserId = Uuid;
pub type Username = String;
pub type UserPassword = String;

impl User {
    pub fn new(id: UserId, username: Username, password: UserPassword) -> User {
        User {
            id,
            username,
            password,
        }
    }
}
