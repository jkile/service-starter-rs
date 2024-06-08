use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::fmt::Debug;
use uuid::Uuid;
use validator::Validate;

use crate::permissions::{Permission, PermissionsType};

// Used for internal typing and accessing sensitive data
#[derive(Deserialize, Serialize, Clone, Validate)]
pub struct User {
    pub id: UserId,
    #[validate(length(min = 2, max = 255))]
    pub username: Username,
    #[validate(length(min = 8, max = 255))]
    pub password: Option<String>,
    pub access_token: Option<String>,
    pub permissions: Permission,
}

pub type UserId = Uuid;
pub type Username = String;
pub type UserPermissionsList = Vec<Permission>;

// External facing user object. Prefer using this whenever possible
#[derive(Serialize, Deserialize, Debug)]
pub struct UserExternal {
    pub id: UserId,
    pub username: String,
    pub permissions: Permission,
}

#[derive(Debug, FromRow, Deserialize)]
pub struct DbUser {
    pub id: String,
    pub username: Username,
    pub password: Option<String>,
    pub access_token: Option<String>,
    pub permissions_type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Credentials {
    Password(PasswordCredentials),
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct PasswordCredentials {
    #[validate(length(min = 2, max = 255))]
    pub username: String,
    #[validate(length(min = 2, max = 255))]
    pub password: String,
    pub next: Option<String>,
}

impl User {
    pub fn new(
        id: UserId,
        username: Username,
        password: Option<String>,
        access_token: Option<String>,
        permissions: Permission,
    ) -> User {
        User {
            id,
            username,
            password,
            access_token,
            permissions,
        }
    }
}

impl From<DbUser> for User {
    fn from(user: DbUser) -> User {
        User {
            id: Uuid::parse_str(&user.id.as_str()).unwrap(),
            username: user.username,
            password: user.password,
            access_token: user.access_token,
            permissions: Permission::from(user.permissions_type),
        }
    }
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .field("access_token", &"[redacted]")
            .field("permissions", &self.permissions)
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

impl From<User> for UserExternal {
    fn from(user: User) -> Self {
        UserExternal {
            id: user.id,
            username: user.username,
            permissions: user.permissions,
        }
    }
}

impl UserExternal {
    pub fn new(id: UserId, username: String, permissions: Permission) -> UserExternal {
        UserExternal {
            id,
            username,
            permissions,
        }
    }
}
