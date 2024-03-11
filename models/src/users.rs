use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: UserId,
    pub username: Username,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct SessionToken {
    pub session_token: String,
    pub user_id: UserId,
}

pub type UserId = Uuid;
pub type Username = String;

impl User {
    pub fn new(id: UserId, username: Username) -> User {
        User { id, username }
    }
}
