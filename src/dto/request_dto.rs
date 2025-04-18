use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRq {
    pub username: String,
    pub password: String,
}