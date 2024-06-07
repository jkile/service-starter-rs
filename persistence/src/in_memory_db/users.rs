// use std::{borrow::Borrow, hash::Hash};

// use axum_login::axum::async_trait;
// use models::users::{User, UserId, Username};
// use utils::error::ApplicationError;

// use crate::users::UsersTable;

// use super::in_memory_db::InMemoryDb;
// use super::in_memory_db::TableNames;

// #[async_trait]
// impl<UserId, User> UsersTable for InMemoryDb<UserId, User>
// where
//     K: Hash + Eq,
// {
//     async fn get_user_by_id(&self, id: UserId) -> Result<User, ApplicationError> {
//         let store = self.store.blocking_lock_owned().borrow().get_mut();
//         if let Some(table) = store.get(&TableNames::Users) {
//             if let Some(user) = table.rows.borrow().get(&id) {
//                 return Ok(user);
//             } else {
//                 return Err(ApplicationError::ResourceNotFound(
//                     "No user found for provided id".to_string(),
//                 ));
//             }
//         } else {
//             return Err(ApplicationError::SqlError(
//                 "Failed to locate table in InMemoryDb".to_string(),
//             ));
//         }
//     }
//     async fn create_user(&self, user: User) -> Result<User, ApplicationError> {}
//     async fn check_username(&self, username: Username) -> Result<i32, ApplicationError> {}
// }
