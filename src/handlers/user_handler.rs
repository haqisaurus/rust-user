use crate::dto::common_dto::PaginationRq;
use crate::dto::error_dto::AppError;
use crate::dto::response_dto::{
    CommonRs, PaginationRs, UserAuditExampleRs, UserAuditJoinExampleRs, UserAuditNestedExampleRs,
};
use crate::models::prelude::{User, UserAudit};
use crate::models::{company, user, user_audit};
use crate::AppState;
use actix_web::{web, Error, HttpResponse};
use futures::future::join_all;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{
    ActiveModelTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};


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
        .map_err(|e|AppError::DbError(e, "".to_string()))?;

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
