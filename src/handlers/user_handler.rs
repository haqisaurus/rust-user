use crate::dto::common_dto::{Claims, PaginationRq};
use crate::dto::error_dto::AppError;
use crate::dto::request_dto::UserMemberRq;
use crate::dto::response_dto::{
    CommonRs, PaginationRs, UserAuditExampleRs, UserAuditJoinExampleRs, UserAuditNestedExampleRs,
};
use crate::models::prelude::{User, UserAudit};
use crate::models::user::ActiveModel;
use crate::models::{company, rel_user_company_role, role, user, user_audit};
use crate::utils::authority_util::authority;
use crate::utils::mail_util::send_email_activation_member;
use crate::AppState;
use actix_web::{web, Error, HttpMessage, HttpRequest, HttpResponse};
use chrono::Local;
use futures::future::join_all;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{
    ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set, TransactionTrait,
};
use std::env;

pub async fn user_save(
    state: web::Data<AppState>,
    req: web::Json<UserMemberRq>,
    http_request: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let conn = &state.conn;
    authority(vec!["user save".to_string()], conn, &http_request).await?;

    let token_data = http_request.extensions().get::<Claims>().cloned().unwrap();
    let username = token_data.sub.clone();
    let txn = conn.begin().await?;
    use bcrypt::{hash, DEFAULT_COST};
    let default_password = "Default123!";
    let password = hash(default_password.to_owned(), DEFAULT_COST);

    // generate activation key
    let hash_id_salt: String = env::var("HASH_ID_SALT").unwrap_or_else(|_| "walla".to_string());
    let hash_ids = hash_ids::HashIds::builder()
        .with_min_length(40)
        .with_salt(&hash_id_salt)
        .finish();

    let activation_key = hash_ids.encode(&[Local::now().timestamp() as u64]);

    if req.id.is_none() {
        let new_data = user::ActiveModel {
            username: Set(req.username.clone()),
            password: Set(password.unwrap().to_string().clone()),
            first_name: Set(req.first_name.clone()),
            last_name: Set(req.last_name.clone()),
            photo: Default::default(),
            activated: Set(false),
            email: Set(req.email.clone()),
            language: Set("ID".into()),
            currency: Set("IDR".into()),
            notification: Default::default(),
            activation_key: Set(activation_key.clone()),
            reset_key: Default::default(),
            reset_date: Default::default(),
            admin: Set(false),
            must_change_password: Set(false),
            enforce_password_policy: Set(false),
            wrong_password_locked: Set(false),
            locked_date: Set(None),
            disable_mobile_android: Set(false),
            disable_mobile_ios: Set(false),
            disable_web: Set(false),
            account_type: Set("MEMBER".to_string()),
            activated_at: Default::default(),
            created_at: Set(Local::now().naive_local()),
            created_by: Set(username.clone()),
            updated_at: Set(None),
            updated_by: Set(None),
            deleted_at: Set(None),
            deleted_by: Set(None),
            ..Default::default()
        };
        let result = new_data.insert(&txn).await;
        if result.is_err() {
            txn.rollback().await?;
            return Err(AppError::DbError(result.err().unwrap(), "".to_string()));
        }
        let user_model = result?;
        // add role member
        let result = role::Entity::find()
            .filter(Expr::col(role::Column::Name).eq("MEMBER".to_string()))
            .one(&txn)
            .await;
        if result.is_err() {
            txn.rollback().await?;
            return Err(AppError::DbError(result.err().unwrap(), "".to_string()));
        }
        let role = result?.unwrap();

        let claims = http_request.extensions().get::<Claims>().cloned().unwrap();
        let company_id = claims.company.clone().parse::<i64>().unwrap().clone();
        let new_data = rel_user_company_role::ActiveModel {
            user_id: Set(user_model.id),
            role_id: Set(role.id),
            company_id: Set(company_id),
            ..Default::default()
        };
        let result = new_data.insert(&txn).await;
        if result.is_err() {
            txn.rollback().await?;
            return Err(AppError::DbError(result.err().unwrap(), "".to_string()));
        }

        // send email
        let result_send_email = send_email_activation_member(&req, &activation_key).await;
        if result_send_email.is_err() {
            txn.rollback().await?;
            return Err(result_send_email.err().unwrap());
        }
    } else {
        let user_id = req.id.unwrap();
        let result = user::Entity::find_by_id(user_id).one(conn).await;
        if result.is_err() {
            txn.rollback().await?;
            return Err(AppError::DbError(result.unwrap_err(), "".to_string()));
        }
        let success = result.is_ok();
        let model = result?;
        if success && model.is_none() {
            txn.rollback().await?;
            return Err(AppError::NotFound(400005, "".to_string()));
        }

        let mut user_model: ActiveModel = model.unwrap().into_active_model();
        user_model.updated_at = Set(Some(Local::now().naive_local()));
        user_model.updated_by = Set(Some(username.clone()));
        user_model.update(conn).await?;
    }

    txn.commit().await?;
    Ok(HttpResponse::Ok().json(CommonRs {
        message: "SUCCESS".to_string(),
        code: "0".to_string(),
        data: "".to_string(),
    }))
}
pub async fn error_handler(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let db = &state.conn;

    // create company
    let new_company = company::ActiveModel {
        name: Set("req.company_name".to_string()),
        description: Set("".to_string()),
        ..Default::default()
    };

    let result = new_company.insert(db).await;
    if result.is_err() {
        return Err(AppError::DbError(result.err().unwrap(), "".to_string()));
    }
    Ok(HttpResponse::Ok().json(result?))
}

pub async fn get_test_join_json(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let db = &state.conn;
    let users_with_logs = user::Entity::find()
        .find_also_related(user_audit::Entity)
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|(user, user_audit)| UserAuditJoinExampleRs {
            id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            status: user_audit
                .as_ref()
                .map_or("".to_string(), |a| a.status.clone()),
            activity: user_audit
                .as_ref()
                .map_or("".to_string(), |a| a.activity.clone()),
            user_agent: user_audit
                .as_ref()
                .map_or("".to_string(), |a| a.user_agent.clone()),
            platform: user_audit
                .as_ref()
                .map_or("".to_string(), |a| a.platform.clone()),
        })
        .collect::<Vec<UserAuditJoinExampleRs>>();

    Ok(HttpResponse::Ok().json(users_with_logs))
}
pub async fn get_test_nested_json(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let db = &state.conn;

    let users = User::find().all(db).await.unwrap();

    let users_with_logs = join_all(users.into_iter().map(|user| {
        let db = db.clone(); // Capture db inside future
        async move {
            let audits = user
                .find_related(UserAudit)
                .order_by_desc(user_audit::Column::CreatedAt)
                .all(&db)
                .await
                .unwrap();

            let logs: Vec<UserAuditExampleRs> = audits
                .into_iter()
                .map(|aud| UserAuditExampleRs {
                    id: aud.id,
                    status: aud.status.clone(),
                    activity: aud.activity.clone(),
                    user_agent: aud.user_agent.clone(),
                    platform: aud.platform.clone(),
                })
                .collect();

            UserAuditNestedExampleRs {
                id: user.id,
                username: user.username,
                first_name: user.first_name,
                last_name: user.last_name,
                logs,
            }
        }
    }))
    .await;

    Ok(HttpResponse::Ok().json(users_with_logs))
}
pub async fn get_users(
    state: web::Data<AppState>,
    query: web::Query<PaginationRq>,
) -> Result<HttpResponse, Error> {
    let db = &state.conn;
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);
    let mut query_builder = user::Entity::find();

    // Filter by name (e.g., LIKE %name%)
    if let Some(ref name) = query.name {
        query_builder = query_builder
            .filter(Expr::col(user::Column::Username).ilike(format!("%{}%", name.clone())));
    }

    // Sorting
    if let Some(ref sort_by) = query.sort_by {
        let order = match query.order.as_deref() {
            Some("desc") => sea_orm::Order::Desc,
            _ => sea_orm::Order::Asc,
        };

        query_builder = match sort_by.as_str() {
            "name" => query_builder.order_by(user::Column::Username, order),
            "email" => query_builder.order_by(user::Column::Password, order),
            "created_at" => query_builder.order_by(user::Column::CreatedAt, order),
            _ => query_builder,
        };
    }

    // Pagination
    let paginator = query_builder.paginate(db, per_page);
    let total_items = paginator.num_items().await.unwrap_or(0);
    let total_pages = paginator.num_pages().await.unwrap_or(0);
    let data = paginator
        .fetch_page(page.saturating_sub(1))
        .await
        .map_err(|e| AppError::DbError(e, "".to_string()))?;

    Ok(HttpResponse::Ok().json(CommonRs {
        message: "SUCCESS".to_string(),
        code: "0".to_string(),
        data: PaginationRs {
            content: data,
            page,
            per_page,
            total_items,
            total_pages,
        },
    }))
}
