use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct PermissionRq {
    pub id: Option<i64>,
    #[validate(length(min = 3,message = "Permission must be greater than 3 chars"))]
    pub name: String,
    pub description: String,
    pub group: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct LoginRq {
    #[validate(length(min = 3,message = "Username must be greater than 3 chars"))]
    pub username: String,
    pub password: String,
    pub domain: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RegisterRq {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    pub username: String,
    pub password: String,
    #[validate(length(min = 4 ,message= "Company name less than 4 char"))]
    #[serde(rename = "companyName")]
    pub company_name: String,
    #[validate(length(min = 1 ,message= "Domain cannot be empty"))]
    pub domain: String,
}