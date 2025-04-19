use crate::dto::request_dto::LoginRq;
use crate::dto::response_dto::CommonRs;
use crate::models::prelude::{User, UserAudit};
use crate::models::user::Model;
use crate::models::{user, user_audit};
use crate::utils::misc_util::detect_os;
use actix_web::web::Json;
use actix_web::{Error, HttpRequest, HttpResponse};
use chrono::Local;
use sea_orm::prelude::Expr;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};

pub async fn get_current_user_by_username(
    login_req: &Json<LoginRq>,
    conn: &DatabaseConnection,
) -> Result<Model, Result<HttpResponse, Error>> {
    let condition = Expr::col(user::Column::Username).eq(login_req.username.clone());
    let result_model = User::find().filter(condition).one(conn).await;
    let current_user = result_model.as_ref().map(|s| s.clone()).unwrap();
    if result_model.is_err() {
        let response = CommonRs {
            code: "5000".to_string(),
            message: result_model.err().unwrap().to_string(),
            data: "".to_string(),
        };
        return Err(Ok(HttpResponse::InternalServerError().json(response)));
    }
    if result_model.unwrap().is_none() {
        let response = CommonRs {
            code: "4001".to_string(),
            message: "Invalid username or password".to_string(),
            data: "".to_string(),
        };
        return Err(Ok(HttpResponse::Unauthorized().json(response)));
    }
    Ok(current_user.unwrap())
}

pub async fn get_unique_by_username(
    username: &String,
    conn: &DatabaseConnection,
) -> Result<Option<Model>, Result<HttpResponse, Error>> {
    let condition = Expr::col(user::Column::Username).eq(username.clone());
    let result_model = User::find().filter(condition).one(conn).await;
    if let Some(value) = throw_response_error(&result_model) {
        return value;
    }

    Ok(result_model.unwrap())
}
pub async fn get_unique_by_email(
    email: &String,
    conn: &DatabaseConnection,
) -> Result<Option<Model>, Result<HttpResponse, Error>> {
    let condition = Expr::col(user::Column::Email).eq(email.clone());
    let result_model = User::find().filter(condition).one(conn).await;
    let current_user = result_model.as_ref().map(|s| s.clone()).unwrap();
    if let Some(value) = throw_response_error(&result_model) {
        return value;
    }

    Ok(current_user)
}

pub async fn create_audit_log(
    conn: &DatabaseConnection,
    http_req: &HttpRequest,
    req_data: &LoginRq,
) -> i64 {
    let user_agent = http_req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let ip = http_req
        .connection_info().peer_addr().unwrap_or("").to_string();

    let platform = detect_os(&user_agent);

    let audit_data = user_audit::ActiveModel {
        id: Default::default(),
        user_id: Default::default(),
        username: Set(req_data.username.clone()),
        created_at: Set(Local::now().naive_local()),
        status: Set("FAILED".to_string()),
        user_agent: Set(user_agent.parse().unwrap()),
        ip: Set(ip),
        expired_at: Set(Local::now().naive_local()),
        token: Set("".to_string()),
        refresh_token: Set("".to_string()),
        platform: Set(platform.parse().unwrap()),
        activity: Set("LOGIN".to_string()),
    };
    let result = audit_data.insert(conn).await.unwrap();
    result.id
}

pub async fn update_audit_log(
    conn: &DatabaseConnection,
    token: &String,
    refresh_token: &String,
    user_id: i64,
    log_id: i64,
) {
    let result_data: Option<user_audit::Model> =
        UserAudit::find_by_id(log_id).one(conn).await.unwrap();
    let mut audit_data: user_audit::ActiveModel = result_data.unwrap().into();
    audit_data.user_id = Set(user_id);
    audit_data.token = Set(token.clone());
    audit_data.refresh_token = Set(refresh_token.clone());
    audit_data.status = Set("SUCCESS".to_string());
    audit_data.update(conn).await.unwrap();
}

fn throw_response_error(
    result_model: &Result<Option<Model>, DbErr>,
) -> Option<Result<Option<Model>, Result<HttpResponse, Error>>> {
    let cloned_data = result_model.as_ref().map(|s| s.clone());
    if cloned_data.is_err() {
        let response = CommonRs {
            code: "5000".to_string(),
            message: cloned_data.err().unwrap().to_string(),
            data: "".to_string(),
        };
        return Some(Err(Ok(HttpResponse::InternalServerError().json(response))));
    }
    None
}
