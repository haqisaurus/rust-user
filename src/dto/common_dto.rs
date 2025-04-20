use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PaginationRq {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub name: Option<String>,     // for filtering
    pub value: Option<String>,     // for filtering
    pub sort_by: Option<String>,  // e.g. "name", "email"
    pub order: Option<String>,    //
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub company: String,
    pub(crate) iat: i64,
    pub(crate) exp: i64,
}

