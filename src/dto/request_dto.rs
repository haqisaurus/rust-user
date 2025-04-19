use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRq {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}