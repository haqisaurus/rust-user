use crate::dto::request_dto::LoginRq;
use crate::dto::response_dto::CommonRs;
use crate::models::prelude::User;
use crate::models::user;
use crate::models::user::Model;
use actix_web::web::Json;
use actix_web::{Error, HttpResponse};
use sea_orm::prelude::Expr;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter};

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
        return  Err(Ok(HttpResponse::InternalServerError().json(response)));
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

fn throw_response_error(result_model: &Result<Option<Model>, DbErr>) -> Option<Result<Option<Model>, Result<HttpResponse, Error>>> {
    let cloned_data = result_model.as_ref().map(|s|s.clone());
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
