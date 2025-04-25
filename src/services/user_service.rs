use crate::dto::error_dto::AppError;
use crate::dto::request_dto::LoginRq;
use crate::models::prelude::{User, UserAudit};
use crate::models::user::Model;
use crate::models::{user, user_audit};
use crate::utils::misc_util::detect_os;
use actix_web::web::Json;
use actix_web::HttpRequest;
use chrono::Local;
use sea_orm::prelude::Expr;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn get_current_user_by_username(
    login_req: &Json<LoginRq>,
    conn: &DatabaseConnection,
) -> Result<Model, AppError> {
    let condition = Expr::col(user::Column::Username).eq(login_req.username.clone());
    let result_model = User::find().filter(condition).one(conn).await;
    let current_user = result_model.as_ref().map(|s| s.clone()).unwrap();
    if result_model.is_err() {
        return Err(AppError::InternalError(
            10002,
            result_model.err().unwrap().to_string(),
        ));
    }
    if result_model?.is_none() {
        return Err(AppError::Unauthorized(
            10002,
            "Invalid username or password".to_string(),
        ));
    }
    Ok(current_user.unwrap())
}

pub async fn get_unique_by_username(
    username: &String,
    conn: &DatabaseConnection,
) -> Result<Option<Model>, AppError> {
    let condition = Expr::col(user::Column::Username).eq(username.clone());
    let result_model = User::find().filter(condition).one(conn).await;
    if result_model.is_err() {
        let error = result_model.unwrap_err();
        return Err(AppError::DbError(error, "".to_string()));
    }

    Ok(result_model?)
}
pub async fn get_unique_by_email(
    email: &String,
    conn: &DatabaseConnection,
) -> Result<Option<Model>, AppError> {
    let condition = Expr::col(user::Column::Email).eq(email.clone());
    let result_model = User::find().filter(condition).one(conn).await;
    if result_model.is_err() {
        let error = result_model.unwrap_err();
        return Err(AppError::DbError(error, "".to_string()));
    }

    Ok(result_model?)
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
        .connection_info()
        .peer_addr()
        .unwrap_or("")
        .to_string();

    let platform = detect_os(&user_agent);

    let audit_data = user_audit::ActiveModel {
        id: Default::default(),
        user_id: Default::default(),
        username: Set(req_data.username.clone()),
        created_at: Set(Local::now().naive_local()),
        status: Set("FAILED".to_string()),
        user_agent: Set(user_agent.parse().unwrap()),
        ip: Set(ip),
        expired_at: Set(Some(Default::default())),
        token: Set(Some(Default::default())),
        refresh_token: Set(Some(Default::default())),
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
    audit_data.token = Set(Some(token.clone()));
    audit_data.refresh_token = Set(Some(refresh_token.clone()));
    audit_data.status = Set("SUCCESS".to_string());
    audit_data.update(conn).await.unwrap();
}