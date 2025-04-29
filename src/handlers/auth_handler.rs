use crate::dto::common_dto::Claims;
use crate::dto::error_dto::AppError;
use crate::dto::request_dto::{LoginRq, RefreshTokenrRq, RegisterRq};
use crate::dto::response_dto::{CommonRs, CompanyRegisterListRs, LoginRs};
use crate::models::{company, rel_user_company_role};
use crate::models::{user, user_audit};
use crate::services::company_service::get_company_by_domain;
use crate::services::user_service::{
    create_audit_log, get_current_user_by_username_is_active, get_unique_by_email,
    get_unique_by_username, update_audit_log,
};
use crate::utils::error_util::serialize_error;
use crate::utils::mail_util::send_email_activation;
use crate::utils::misc_util::detect_os;
use crate::AppState;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use chrono::{DateTime, Duration};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::sea_query::{Alias, Expr, IntoCondition};
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, JoinType, QueryFilter,
    QuerySelect, RelationTrait, Set, TransactionTrait,
};
use std::env;
use validator::Validate;

pub async fn company_list(
    data: web::Data<AppState>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let conn = &data.conn;
    let token_data = http_req.extensions().get::<Claims>().cloned().unwrap();
    let user_id = token_data.iss.clone().parse::<i64>().unwrap().clone();

    let company_list = company::Entity::find()
        .join_as(
            JoinType::InnerJoin,
            rel_user_company_role::Relation::Company
                .def()
                .rev()
                .on_condition(move |_left, right| {
                    Expr::col((right, rel_user_company_role::Column::UserId))
                        .eq(user_id)
                        .into_condition()
                }),
            Alias::new("company_rel"),
        )
        .all(conn)
        .await?;

    let content = company_list
        .into_iter()
        .map(|company| CompanyRegisterListRs {
            id: company.id,
            name: company.name,
            logo: company.logo,
        })
        .collect::<Vec<CompanyRegisterListRs>>();

    Ok(HttpResponse::Ok().json(CommonRs {
        message: "SUCCESS".to_string(),
        code: "0".to_string(),
        data: content,
    }))
}

pub async fn post_login(
    data: web::Data<AppState>,
    req: web::Json<LoginRq>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let conn = &data.conn;
    // create log data
    let log_id = create_audit_log(&conn, &http_req, &req).await;

    // check data on db
    let result = get_current_user_by_username_is_active(&req, conn).await;
    if result.is_err() {
        return Err(result.err().unwrap());
    }
    let current_user = result?;

    let result = get_company_by_domain(&req.domain, conn).await;
    if result.is_err() {
        return Err(result.err().unwrap());
    }
    let company = result?.unwrap();

    // check password
    let password = current_user.password.clone();
    use bcrypt::verify;
    let valid = verify(&req.password, &password);
    if valid.is_err() {
        return Err(AppError::Unauthorized(40102, "".to_string()));
    }

    // company check
    let result = get_company_by_domain(&req.domain, conn).await;
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // generate token
    let mut header = Header::new(Algorithm::RS256);
    header.typ = Some("JWT".to_string());
    let private_key: String = env::var("JWT_SECRET").unwrap_or_else(|_| "walla".to_string());
    let client_id: String = String::from(current_user.id.to_string());
    let username: String = String::from(current_user.username);
    let company_id: String = String::from(company.id.to_string());

    let now = Local::now();
    let iat = now.timestamp();
    let exp = (now + Duration::hours(1)).timestamp();
    let claims = Claims {
        iss: client_id.clone(),
        sub: username.clone(),
        company: company_id.to_owned(),
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
    let refresh_token = hash_ids.encode(&[exp_refresh as u64, exp as u64]);

    // update log data
    update_audit_log(conn, &token, &refresh_token, current_user.id, log_id).await;

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

pub async fn refresh_token(
    data: web::Data<AppState>,
    req: web::Json<RefreshTokenrRq>,
    http_request: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let conn = &data.conn;

    // decode and compare
    let hash_id_salt: String = env::var("HASH_ID_SALT").unwrap_or_else(|_| "walla".to_string());
    let hash_ids = hash_ids::HashIds::builder()
        .with_min_length(60)
        .with_salt(&hash_id_salt)
        .finish();

    let refresh_token = hash_ids.decode(&req.refresh_token).unwrap();

    let token = http_request
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .unwrap_or("");

    let token_cond = Expr::col(user_audit::Column::Token).eq(token);
    let refresh_cond = Expr::col(user_audit::Column::RefreshToken).eq(req.refresh_token.clone());
    let condition = token_cond.and(refresh_cond).into_condition();
    let result = user_audit::Entity::find().filter(condition).one(conn).await;
    if result.is_err() {
        return Err(AppError::DbError(result.err().unwrap(), "".to_string()));
    }
    let now = Local::now();
    let expired_column = result?.unwrap().expired_at.unwrap();
    if (refresh_token[0] as i64) < now.timestamp()
        && (refresh_token[1] as i64) == expired_column.and_utc().timestamp()
    {
        return Err(AppError::Unauthorized(401000, "".to_string()));
    }
    let private_key = env::var("JWT_SECRET").unwrap_or_else(|_| "walla".into());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;
    let claims = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(private_key.as_ref()),
        &validation,
    ) {
        Ok(token_data) => {
            println!("{:?}", token_data.claims);
            Ok(token_data)
        }
        Err(err) => Err(err.to_string()),
    };
    let claims = claims.unwrap().claims;
    let username = claims.sub.clone();
    let user_id = claims.iss.clone().parse::<i64>().unwrap().clone();
    let company_id = claims.company.clone().parse::<i64>().unwrap().clone();

    let private_key: String = env::var("JWT_SECRET").unwrap_or_else(|_| "walla".to_string());
    let iat = now.timestamp();
    let exp = (now + Duration::hours(1)).timestamp();
    let claims = Claims {
        iss: user_id.to_string(),
        sub: username.clone(),
        company: company_id.to_string(),
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
    let refresh_token = hash_ids.encode(&[exp_refresh as u64, exp as u64]);
    let user_agent = http_request
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let ip = http_request
        .connection_info()
        .peer_addr()
        .unwrap_or("")
        .to_string();

    let platform = detect_os(&user_agent);
    let audit_data = user_audit::ActiveModel {
        id: Default::default(),
        user_id: Set(user_id),
        username: Set(claims.sub.clone()),
        created_at: Set(Local::now().naive_local()),
        status: Set("SUCCESS".to_string()),
        user_agent: Set(user_agent.parse().unwrap()),
        ip: Set(ip),
        expired_at: Set(Some(
            DateTime::from_timestamp(exp, 0).unwrap().naive_local(),
        )),
        token: Set(Some(token.clone())),
        refresh_token: Set(Some(refresh_token.clone())),
        platform: Set(platform.parse().unwrap()),
        activity: Set("REFRESH_TOKEN".to_string()),
    };
    audit_data.insert(conn).await?;
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
) -> Result<HttpResponse, AppError> {
    if req.validate().is_err() {
        let err = req.validate().unwrap_err();
        let messages = serialize_error(&err);
        return Err(AppError::ValidationRequest(
            400000,
            serde_json::json!(messages),
        ));
    }
    let conn = &data.conn;
    let conn_cloned = conn.clone();
    let txn = conn_cloned.begin().await?;

    // check username and email used
    let username_check_result = get_unique_by_username(&req.username, &conn).await;
    if username_check_result.is_err() {
        return Err(username_check_result.unwrap_err());
    }
    if username_check_result.is_ok_and(|t| t.is_some()) {
        return Err(AppError::BadRequest(400001, "".to_string()));
    }

    let email_check_result = get_unique_by_email(&req.email, &conn).await;
    if email_check_result.is_err() {
        return Err(email_check_result.unwrap_err());
    }
    if email_check_result.is_ok_and(|t| t.is_some()) {
        return Err(AppError::BadRequest(400002, "".to_string()));
    }

    // saving data
    use bcrypt::{hash, DEFAULT_COST};
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
        account_type: Set("OWNER".to_string()),
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
        updated_at: Set(None),
        created_by: Set("SYSTEM".to_string()),
        updated_by: Set(None),
        activated: Set(false),
        activated_at: Set(None),

        ..Default::default() // all other attributes are `NotSet`
    };
    let result = new_user.insert(&txn).await;
    let user = result.map_err(|e| e);

    let new_user_id = user?.id;

    let result = get_company_by_domain(&req.domain, conn).await;
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    if result.is_ok_and(|t| t.is_some()) {
        return Err(AppError::BadRequest(400003, "".to_string()));
    }

    // create company
    let new_company = company::ActiveModel {
        name: Set(req.company_name.clone()),
        description: Set("".to_string()),
        user_id: Set(new_user_id),
        domain: Set(req.domain.clone()),
        status: Set("ACTIVE".to_string()),
        created_at: Set(Local::now().naive_local()),
        updated_at: Set(Local::now().naive_local()),
        created_by: Set("SYSTEM".to_string()),
        updated_by: Set("SYSTEM".to_string()),
        ..Default::default()
    };

    let result = new_company.insert(&txn).await;
    if result.is_err() {
        return Err(AppError::DbError(result.err().unwrap(), "".to_string()));
    }

    // send email
    let result_send_email = send_email_activation(&req, &activation_key).await;
    if result_send_email.is_err() {
        txn.rollback().await?;
        return Err(result_send_email.err().unwrap());
    }
    txn.commit().await?;

    Ok(HttpResponse::Ok().json(CommonRs {
        message: "SUCCESS".to_string(),
        code: "0".to_string(),
        data: "".to_string(),
    }))
}

pub async fn activation_user(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let conn = &data.conn;
    let activation_key = path.into_inner();
    let condition = Expr::col(user::Column::ActivationKey).eq(activation_key);

    let result = user::Entity::find()
        .filter(condition)
        .filter(user::Column::DeletedAt.is_null())
        .one(conn)
        .await;
    if result.is_err() {
        return Err(AppError::DbError(result.unwrap_err(), "".to_string()));
    }
    let success = result.is_ok();
    let model = result?;
    if success && model.is_none() {
        return Err(AppError::NotFound(400000, "".to_string()));
    }
    let mut user_model = model.unwrap().into_active_model();
    user_model.activated = Set(true);
    user_model.activated_at = Set(Some(Local::now().naive_local()));
    user_model.activation_key = Set("".to_string());
    user_model.updated_at = Set(Some(Local::now().naive_local()));
    user_model.updated_by = Set(Some("SYSTEM".to_string()));
    user_model.update(conn).await?;

    Ok(HttpResponse::Ok().json(CommonRs {
        message: "SUCCESS".to_string(),
        code: "0".to_string(),
        data: "".to_string(),
    }))
}
