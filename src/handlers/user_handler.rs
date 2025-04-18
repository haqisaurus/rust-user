use crate::AppState;
use crate::dto::response_dto::{CommonRs, PaginationRs};
use crate::models::common_dto::PaginationRq;
use crate::models::user;
use actix_web::{Error, HttpResponse, web};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

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
            .filter(Expr::col(user::Column::Login).ilike(format!("%{}%", name.clone())));
    }

    // Sorting
    if let Some(ref sort_by) = query.sort_by {
        let order = match query.order.as_deref() {
            Some("desc") => sea_orm::Order::Desc,
            _ => sea_orm::Order::Asc,
        };

        query_builder = match sort_by.as_str() {
            "name" => query_builder.order_by(user::Column::Login, order),
            "email" => query_builder.order_by(user::Column::Password, order),
            "created_at" => query_builder.order_by(user::Column::CreatedDate, order),
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
        .map_err(actix_web::error::ErrorInternalServerError)?;

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
