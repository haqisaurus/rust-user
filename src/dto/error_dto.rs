use crate::config::constant::ERRORS;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use sea_orm::DbErr;
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize)]
pub struct ErrorResponse<T> {
    pub code: u32,
    pub message: T,
    pub data: String,
}

#[derive(Debug)]
pub enum AppError {
    DbError(DbErr, String),
    NotFound(u32, String),
    ValidationRequest(u32, serde_json::Value),
    BadRequest(u32, String),
    InternalError(u32, String),
    Unauthorized(u32, String),
    Forbidden(u32, String),
}

impl Display for AppError {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, code, message): (StatusCode, u32, serde_json::Value) = match self {
            AppError::DbError(e, msg) => {
                if msg.is_empty() {
                    (StatusCode::INTERNAL_SERVER_ERROR, 500001u32, serde_json::json!(e.to_string()))
                } else {
                    (StatusCode::INTERNAL_SERVER_ERROR, 500001u32, serde_json::json!(msg.to_string()))
                }
            },
            AppError::NotFound(code,  message) => {
                if let Some(item) = ERRORS.iter().find(|(id, _)| id == code) {
                    (StatusCode::NOT_FOUND, code.clone(), serde_json::json!(item.clone().1.to_string()))
                } else {
                    (StatusCode::NOT_FOUND, 404000u32, if message.is_empty() {serde_json::json!("Not found".to_string())} else { serde_json::json!(message.to_string()) })
                }
            }
            AppError::BadRequest(code,  message) => {
                if let Some(item) = ERRORS.iter().find(|(id, _)| id == code) {
                    (StatusCode::BAD_REQUEST, code.clone(), serde_json::json!(item.clone().1.to_string()))
                } else {
                     (StatusCode::BAD_REQUEST, 400000u32, if message.is_empty() { serde_json::json!("Bad Request".to_string()) } else { serde_json::json!(message.to_string()) })
                }
            }
            AppError::InternalError(code,  message) => {
                if let Some(item) = ERRORS.iter().find(|(id, _)| id == code) {
                    (StatusCode::INTERNAL_SERVER_ERROR, code.clone(), serde_json::json!(item.clone().1.to_string()))
                } else {
                    (StatusCode::INTERNAL_SERVER_ERROR, 500000u32, if message.is_empty() { serde_json::json!("Bad Request".to_string())} else { serde_json::json!(message.to_string()) })
                }
            }
            AppError::Unauthorized(code,  msg) => {
                if let Some(item) = ERRORS.iter().find(|(id, _)| id == code) {
                    (StatusCode::UNAUTHORIZED, code.clone(), serde_json::json!(item.clone().1.to_string()))
                } else {
                    (StatusCode::UNAUTHORIZED, 401000u32, if msg.is_empty() { serde_json::json!("Unauthorized Request".to_string()) } else { serde_json::json!(msg.to_string()) })
                }
            }
            AppError::Forbidden(code,  msg) => {
                if let Some(item) = ERRORS.iter().find(|(id, _)| id == code) {
                    (StatusCode::FORBIDDEN, code.clone(), serde_json::json!(item.clone().1.to_string()))
                } else {
                    (StatusCode::FORBIDDEN, 403000u32, if msg.is_empty() { serde_json::json!("Forbidden Request".to_string()) } else { serde_json::json!(msg.to_string()) })
                }
            }
            AppError::ValidationRequest(code, message ) => {
                (StatusCode::BAD_REQUEST, code.to_owned(), serde_json::json!(message))
            }
        };

        // println!(" - {} - {} - {}", status, code, message);
        let body = ErrorResponse {
            code,
            message,
            data: "".to_string(),
        };

        HttpResponse::build(status).json(body)
    }
}

impl From<DbErr> for AppError {
    fn from(e: DbErr ) -> Self {
        AppError::DbError(e, "".to_owned())
    }
}