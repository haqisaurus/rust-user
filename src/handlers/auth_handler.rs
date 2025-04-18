use crate::AppState;
use crate::dto::request_dto::LoginRq;
use crate::dto::response_dto::{CommonRs, LoginRs};
use crate::models::common_dto::Claims;
use crate::models::prelude::User;
use crate::models::user;
use actix_web::{Error, HttpResponse, web};
use chrono::Duration;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use sea_orm::sea_query::Expr;
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{EntityTrait, QueryFilter};
use std::env;

pub async fn login(
    data: web::Data<AppState>,
    login_req: web::Json<LoginRq>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let condition = Expr::col(user::Column::Login).eq(login_req.username.clone());
    let current_user = User::find().filter(condition).one(conn).await;
    if current_user.is_err() {
        let response = CommonRs {
            code: "4001".to_string(),
            message: "Invalid username or password".to_string(),
            data: "".to_string(),
        };
        return Ok(HttpResponse::Unauthorized().json(response));
    }
    let password = current_user.unwrap().unwrap().password.unwrap();
    use bcrypt::verify;

    let valid = verify(&login_req.password, &password);
    if valid.is_err() || !valid.unwrap() {
        let response = CommonRs {
            code: "4001".to_string(),
            message: "Invalid username or password".to_string(),
            data: "".to_string(),
        };
        return Ok(HttpResponse::Unauthorized().json(response));
    }
    let mut header = Header::new(Algorithm::RS256);
    header.typ = Some("JWT".to_string());
    let private_key: String = env::var("JWT_SECRET").unwrap_or_else(|_| "walla".to_string());
    let client_id: String = String::from("asdf");
    let service_account: String = String::from("asdf");

    let now = Local::now();
    let iat = now.timestamp();
    let exp = (now + Duration::hours(1)).timestamp();
    let claims = Claims {
        iss: client_id.clone(),
        sub: service_account.clone(),
        company: "ACME".to_owned(),
        iat,
        exp,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(private_key.as_ref()),
    )
    .unwrap();
    let hash_id_salt: String = env::var("HASH_ID_SALT").unwrap_or_else(|_| "walla".to_string());

    let hash_ids = hash_ids::HashIds::builder()
        .with_min_length(60)
        .with_salt(&hash_id_salt)
        .finish();
    let refresh_token = hash_ids.encode(&[exp as u64]);

    Ok(HttpResponse::Ok().json(CommonRs {
        message: "SUCCESS".to_string(),
        code: "0".to_string(),
        data: LoginRs {
            token,
            refresh_token,
            expiration: exp,
        },
    }))
}
