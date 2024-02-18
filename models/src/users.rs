use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    id: UserId,
    username: Username,
    password: UserPassword,
}

pub type UserId = String;
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
