use crate::AppState;
use crate::dto::request_dto::{LoginRq, RegisterRq};
use crate::dto::response_dto::{CommonRs, LoginRs};
use crate::dto::common_dto::Claims;
use crate::models::user;
use crate::services::user_service::{
    create_audit_log, get_current_user_by_username, get_unique_by_email, get_unique_by_username,
    update_audit_log,
};
use crate::utils::mail_util::send_email_activation;
use actix_web::{Error, HttpRequest, HttpResponse, web};
use chrono::Duration;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{ActiveModelTrait, Set, TransactionTrait};
use std::env;

pub async fn post_login(
    data: web::Data<AppState>,
    req: web::Json<LoginRq>,
    http_req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    // create log data
    let log_id = create_audit_log(&conn, &http_req, &req).await;

    // check data on db
    let result_model = match get_current_user_by_username(&req, conn).await {
        Ok(value) => value,
        Err(value) => return value,
    };

    // check password
    let current_user = result_model;
    let password = current_user.password;
    use bcrypt::verify;

    let valid = verify(&req.password, &password);
    if valid.is_err() || !valid.unwrap() {
        let response = CommonRs {
            code: "4001".to_string(),
            message: "Invalid username or password".to_string(),
            data: "".to_string(),
        };
        return Ok(HttpResponse::Unauthorized().json(response));
    }

    // generate token
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

    // generate refresh token
    let hash_id_salt: String = env::var("HASH_ID_SALT").unwrap_or_else(|_| "walla".to_string());
    let hash_ids = hash_ids::HashIds::builder()
        .with_min_length(60)
        .with_salt(&hash_id_salt)
        .finish();

    let exp_refresh = (now + Duration::hours(4)).timestamp();
    let refresh_token = hash_ids.encode(&[exp_refresh as u64]);

    // update log data
    update_audit_log(
        conn,
        &token,
        &refresh_token,
        current_user.id,
        log_id,
    )
    .await;

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

pub async fn post_register(
    data: web::Data<AppState>,
    req: web::Json<RegisterRq>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let conn_cloned = conn.clone();
    let txn = conn_cloned.begin().await.unwrap();

    // check username and email used
    let username_check_result = get_unique_by_username(&req.username, &conn).await;
    if username_check_result.is_err() {
        return username_check_result.unwrap_err();
    }
    if username_check_result.unwrap().is_some() {
        let response = CommonRs {
            code: "4000".to_string(),
            message: "Username already taken please login".to_string(),
            data: "".to_string(),
        };
        return Ok(HttpResponse::Unauthorized().json(response));
    }
    let email_check_result = get_unique_by_email(&req.email, &conn).await;
    if email_check_result.is_err() {
        return email_check_result.unwrap_err();
    }
    if email_check_result.unwrap().is_some() {
        let response = CommonRs {
            code: "4000".to_string(),
            message: "Email already exist please login".to_string(),
            data: "".to_string(),
        };
        return Ok(HttpResponse::Unauthorized().json(response));
    }

    // saving data
    use bcrypt::{DEFAULT_COST, hash};
    let password = hash(req.password.clone(), DEFAULT_COST);

    // generate activation key
    let hash_id_salt: String = env::var("HASH_ID_SALT").unwrap_or_else(|_| "walla".to_string());
    let hash_ids = hash_ids::HashIds::builder()
        .with_min_length(40)
        .with_salt(&hash_id_salt)
        .finish();

    let activation_key = hash_ids.encode(&[Local::now().timestamp() as u64]);

    let new_user = user::ActiveModel {
        username: Set(req.username.clone()),
        password: Set(password.unwrap().clone()),
        email: Set(req.email.clone()),
        first_name: Set(req.first_name.clone()),
        last_name: Set(req.last_name.clone()),
        photo: Set(None),
        language: Set("ID".to_string()),
        currency: Set("IDR".to_string()),
        notification: Set(true),
        activation_key: Set(activation_key.to_string().clone()),
        reset_key: Set(None),
        account_type: Set("".to_string()),
        reset_date: Set(None),
        admin: Set(false),
        must_change_password: Set(false),
        enforce_password_policy: Set(false),
        wrong_password_locked: Set(false),
        locked_date: Set(None),
        disable_mobile_android: Set(false),
        disable_mobile_ios: Set(false),
        disable_web: Set(false),
        created_at: Set(Local::now().naive_local()),
        updated_at: Set(Local::now().naive_local()),
        created_by: Set("SYSTEM".to_string()),
        updated_by: Set("SYSTEM".to_string()),
        activated: Set(false),
        activated_at: Set(None),

        ..Default::default() // all other attributes are `NotSet`
    };

    let result_insert = new_user.insert(&txn).await;
    if result_insert.is_err() {
        let response = CommonRs {
            code: "5000".to_string(),
            message: result_insert.err().unwrap().to_string(),
            data: "".to_string(),
        };
        return Ok(HttpResponse::InternalServerError().json(response));
    }

    // send email
    let result_send_email = send_email_activation(&req, &activation_key).await;
    if result_send_email.is_err() {
        txn.rollback().await.unwrap();
        return Ok(result_send_email.err().unwrap());
    }
    log::info!("{}", result_send_email.unwrap());
    txn.commit().await.unwrap();
    Ok(HttpResponse::Ok().json(CommonRs {
        message: "SUCCESS".to_string(),
        code: "0".to_string(),
        data: "".to_string(),
    }))
}
