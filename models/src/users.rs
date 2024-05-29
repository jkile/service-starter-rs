use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

// Used for internal typing and accessing sensitive data
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct User {
    pub id: UserId,
    pub username: Username,
    pub password: Option<String>,
    pub access_token: Option<String>,
}

// External facing user object. Prefer using this whenever possible
#[derive(Serialize, Deserialize)]
pub struct UserExternal {
    id: UserId,
    username: String,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Credentials {
    Password(PasswordCredentials),
}

#[derive(Debug, Clone, Deserialize)]
pub struct PasswordCredentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

pub type UserId = Uuid;
pub type Username = String;

impl User {
    pub fn new(
        id: UserId,
        username: Username,
        password: Option<String>,
        access_token: Option<String>,
    ) -> User {
        User {
            id,
            username,
            password,
            access_token,
        }
    }
}

impl From<User> for UserExternal {
    fn from(user: User) -> Self {
        UserExternal {
            id: user.id,
            username: user.username,
        }
    }
}

impl UserExternal {
    pub fn new(id: UserId, username: String) -> UserExternal {
        UserExternal { id, username }
    }
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .field("access_token", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = UserId;
    fn id(&self) -> Self::Id {
        self.id
    }
    fn session_auth_hash(&self) -> &[u8] {
        if let Some(access_token) = &self.access_token {
            return access_token.as_bytes();
        }

        if let Some(password) = &self.password {
            return password.as_bytes();
        }

        &[]
    }
}
