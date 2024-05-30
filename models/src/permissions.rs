use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Hash, Deserialize, Serialize, PartialEq, Eq, Clone, sqlx::Type)]
#[sqlx(type_name = "permissions_type", rename_all = "lowercase")]
pub enum PermissionType {
    SuperAdmin,
    Admin,
    User,
}

#[derive(Debug, Hash, FromRow, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Permission {
    pub permission_type: PermissionType,
}

pub type PermissionId = Uuid;

impl Permission {
    pub fn new(perm_type: String) -> Permission {
        let permission_type = match perm_type.as_str() {
            "super_admin" => PermissionType::SuperAdmin,
            "admin" => PermissionType::Admin,
            "user" => PermissionType::User,
            // Default to lowest permission
            _ => PermissionType::User,
        };
        Permission { permission_type }
    }
}

impl From<PermissionType> for Permission {
    fn from(permission_type: PermissionType) -> Self {
        Permission { permission_type }
    }
}
