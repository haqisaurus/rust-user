use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CompanyRegisterListRs {
    pub id: i64,
    pub name: String,
    pub logo: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleRs {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub group: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime>,
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<DateTime>,
    #[serde(rename = "deletedBy")]
    pub deleted_by: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PermissionRs {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub group: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime>,
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<DateTime>,
    #[serde(rename = "deletedBy")]
    pub deleted_by: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct UserAuditJoinExampleRs {
    pub id: i64,
    pub username: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub status: String,
    pub activity: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    pub platform: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserAuditNestedExampleRs {
    pub id: i64,
    pub username: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub logs: Vec<UserAuditExampleRs>
}

#[derive(Serialize, Deserialize)]
pub struct UserAuditExampleRs {
    pub id: i64,
    pub status: String,
    pub activity: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    pub platform: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRs {
    pub token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    pub expiration: i64,
}

#[derive(Serialize, Deserialize)]
pub struct PaginationRs<T> {
    pub content: Vec<T>,
    pub page: u64,
    #[serde(rename = "perPage")]
    pub per_page: u64,
    #[serde(rename = "totalItems")]
    pub total_items: u64,
    #[serde(rename = "totalPages")]
    pub total_pages: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CommonRs<T> {
    pub message: String,
    pub code: String,
    pub data: T,
}

