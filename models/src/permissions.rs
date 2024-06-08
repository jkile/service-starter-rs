use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Hash, Deserialize, Serialize, PartialEq, Eq, Clone, sqlx::Type)]
#[sqlx(type_name = "permissions_type", rename_all = "lowercase")]
pub enum PermissionsType {
    SuperAdmin,
    Admin,
    User,
}

#[derive(Debug, Hash, FromRow, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Permission {
    pub permissions_type: PermissionsType,
}

pub type PermissionId = Uuid;

impl Permission {
    pub fn new(perm_type: String) -> Permission {
        let permissions_type = match perm_type.as_str() {
            "super_admin" => PermissionsType::SuperAdmin,
            "admin" => PermissionsType::Admin,
            "user" => PermissionsType::User,
            // Default to lowest permission
            _ => PermissionsType::User,
        };
        Permission { permissions_type }
    }
}

impl From<PermissionsType> for Permission {
    fn from(permissions_type: PermissionsType) -> Self {
        Permission { permissions_type }
    }
}
