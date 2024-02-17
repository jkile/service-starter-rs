use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: Id,
    username: Username,
    password: Password,
}

type Id = String;
type Username = String;
type Password = String;

impl User {
    pub fn new(id: Id, username: Username, password: Password) -> User {
        User {
            id,
            username,
            password,
        }
    }
}
