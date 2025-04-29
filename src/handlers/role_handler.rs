use crate::dto::common_dto::{Claims, PaginationRq};
use crate::dto::error_dto::AppError;
use crate::dto::request_dto::RoleRq;
use crate::dto::response_dto::{CommonRs, PaginationRs, RoleRs};
use crate::models::role;
use crate::models::role::ActiveModel;
use crate::AppState;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use chrono::Local;
use sea_orm::prelude::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, QueryOrder, Set, TransactionTrait,
};

pub async fn role_save(
    data: web::Data<AppState>,
    req: web::Json<RoleRq>,
    http_request: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let token_data = http_request.extensions().get::<Claims>().cloned().unwrap();
    let username = token_data.sub.clone();

    let conn = &data.conn;
    let txn = conn.begin().await?;

    if req.id.is_none() {
        let new_data = role::ActiveModel {
            name: Set(req.name.clone()),
            description: Set(req.description.clone()),
            group: Set(req.group.clone()),
            created_at: Set(Local::now().naive_local()),
            created_by: Set(username.clone()),
            updated_at: Set(None),
            updated_by: Set(None),
            deleted_at: Set(None),
            deleted_by: Set(None),
            ..Default::default()
        };

        let result = new_data.insert(conn).await;
        if result.is_err() {
            txn.rollback().await?;
            return Err(AppError::DbError(result.err().unwrap(), "".to_string()));
        }
    } else {
        let role_id = req.id.unwrap();
        let result = role::Entity::find_by_id(role_id)
            .one(conn)
            .await;
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

        let mut role_model: ActiveModel = model.unwrap().into_active_model();
        role_model.name = Set(req.name.clone());
        role_model.group = Set(req.group.clone());
        role_model.description = Set(req.description.clone());
        role_model.updated_at = Set(Some(Local::now().naive_local()));
        role_model.updated_by = Set(Some(username.clone()));
        role_model.update(conn).await?;
    }

    txn.commit().await?;
    Ok(HttpResponse::Ok().json(CommonRs {
        message: "SUCCESS".to_string(),
        code: "0".to_string(),
        data: "".to_string(),
    }))
}

pub async fn role_detail(
    data: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let conn = &data.conn;
    let role_id = path.into_inner();
    let result = role::Entity::find_by_id(role_id)
        .one(conn)
        .await;
    if result.is_err() {
        return Err(AppError::DbError(result.unwrap_err(), "".to_string()));
    }

    let res = result?.map(|e| RoleRs {
        id: e.id,
        name: e.name.clone(),
        description: e.description.clone(),
        group: e.group.clone(),
        created_at: e.created_at.clone(),
        created_by: e.created_by.clone(),
        updated_at: e.updated_at.clone(),
        updated_by: e.updated_by.clone(),
        deleted_at: e.deleted_at.clone(),
        deleted_by: e.deleted_by.clone(),
    });

    Ok(HttpResponse::Ok().json(res))
}

pub async fn role_list(
    data: web::Data<AppState>,
    query: web::Query<PaginationRq>,
) -> Result<HttpResponse, AppError> {
    let conn = &data.conn;
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);

    let mut query_builder = role::Entity::find();
    query_builder = query_builder.filter(Expr::col(role::Column::DeletedAt).is_null());

    // Sorting
    if let Some(ref sort_by) = query.sort_by {
        let order = match query.order.as_deref() {
            Some("desc") => sea_orm::Order::Desc,
            _ => sea_orm::Order::Asc,
        };

        query_builder = match sort_by.as_str() {
            "name" => query_builder.order_by(role::Column::Name, order),
            "group" => query_builder.order_by(role::Column::Group, order),
            "created_at" => query_builder.order_by(role::Column::CreatedAt, order),
            _ => query_builder,
        };
    }

    // Pagination
    let paginator = query_builder.paginate(conn, per_page);
    let total_items = paginator.num_items().await.unwrap_or(0);
    let total_pages = paginator.num_pages().await.unwrap_or(0);
    let data = paginator
        .fetch_page(page.saturating_sub(1))
        .await
        .map_err(|e| AppError::DbError(e, "".to_string()))?;

    let content = data
        .into_iter()
        .map(|e| RoleRs {
            id: e.id,
            name: e.name.clone(),
            description: e.description.clone(),
            group: e.group.clone(),
            created_at: e.created_at.clone(),
            created_by: e.created_by.clone(),
            updated_at: e.updated_at.clone(),
            updated_by: e.updated_by.clone(),
            deleted_at: e.deleted_at.clone(),
            deleted_by: e.deleted_by.clone(),
        })
        .collect::<Vec<RoleRs>>();

    Ok(HttpResponse::Ok().json(CommonRs {
        message: "SUCCESS".to_string(),
        code: "0".to_string(),
        data: PaginationRs {
            content,
            page,
            per_page,
            total_items,
            total_pages,
        },
    }))
}

pub async fn role_delete(
    data: web::Data<AppState>,
    path: web::Path<i64>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let token_data = req.extensions().get::<Claims>().cloned().unwrap();
    let username = token_data.sub.clone();

    let conn = &data.conn;
    let role_id = path.into_inner();
    let result = role::Entity::find_by_id(role_id)
        .filter(role::Column::DeletedAt.is_null())
        .one(conn)
        .await;
    if result.is_err() {
        return Err(AppError::DbError(result.unwrap_err(), "".to_string()));
    }
    let success = result.is_ok();
    let model = result?;
    if success && model.is_none() {
        return Err(AppError::NotFound(400005, "".to_string()));
    }

    let mut role_model: ActiveModel = model.unwrap().into_active_model();

    // Update name attribute
    role_model.deleted_by = Set(Some(username.clone()));
    role_model.deleted_at = Set(Some(Local::now().naive_local()));
    role_model.updated_by = Set(Some(username));
    role_model.updated_at = Set(Some(Local::now().naive_local()));
    role_model.update(conn).await?;
    Ok(HttpResponse::Ok().json(CommonRs {
        message: "SUCCESS".to_string(),
        code: "0".to_string(),
        data: "".to_string(),
    }))
}
