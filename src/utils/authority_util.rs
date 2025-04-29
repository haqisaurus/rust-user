use crate::dto::common_dto::Claims;
use crate::dto::error_dto::AppError;
use crate::models::{permission, rel_role_permission, rel_user_company_permission, rel_user_company_role};
use actix_web::{HttpMessage, HttpRequest};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::IntoCondition;
use sea_orm::{DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait};

pub async fn authority(
    permissions: Vec<String>,
    conn: &DatabaseConnection,
    http_request: &HttpRequest,
) -> Result<String, AppError> {
    let claims = http_request.extensions().get::<Claims>().cloned().unwrap();
    let user_id = claims.iss.clone().parse::<i64>().unwrap().clone();
    let company_id = claims.company.clone().parse::<i64>().unwrap().clone();
    for permission_name in permissions {
        let condition = Expr::col(permission::Column::Name).eq(permission_name.clone());
        let result = permission::Entity::find().filter(condition).one(conn).await;

        if result.is_err() {
            return Err(AppError::Forbidden(403001, "".to_string()));
        }
        let permission = result?.unwrap();
        let condition =
            Expr::col((permission::Entity, permission::Column::Id)).eq(permission.id.clone());

        let relation_result = permission::Entity::find()
            .join(
                JoinType::InnerJoin,
                rel_role_permission::Relation::Permission
                    .def()
                    .rev()
                    .on_condition(move |_left, right| {
                        let role_cond =
                            Expr::col((right.clone(), rel_role_permission::Column::RoleId))
                                .equals(crate::models::rel_user_company_role::Column::RoleId);

                        role_cond.into_condition()
                    }),
            )
            .join_rev(
                JoinType::InnerJoin,
                rel_user_company_role::Entity::belongs_to(crate::models::rel_role_permission::Entity)
                    .from(crate::models::rel_user_company_role::Column::RoleId)
                    .to(rel_role_permission::Column::RoleId)
                    .on_condition(move |left, _right| {
                            let user_con =
                                Expr::col((left.clone(), rel_user_company_role::Column::UserId))
                                    .eq(user_id);
                            let company_con =
                                Expr::col((left.clone(), rel_user_company_role::Column::CompanyId))
                                    .eq(company_id);

                            user_con.and(company_con).into_condition()
                    })
                    .into()

            )
            .filter(condition.clone())
            .one(conn)
            .await;
        if relation_result.is_err() {
            return Err(AppError::Forbidden(403000, "".to_string()));
        }

        // check on user company permission table
        if relation_result.is_ok_and(|e|e.is_none()) {
            let relation_result = permission::Entity::find()
                .join(
                    JoinType::InnerJoin,
                    rel_user_company_permission::Relation::Permission
                        .def()
                        .rev()
                        .on_condition(move |_left, right| {
                            let user_con =
                                Expr::col((right.clone(), rel_user_company_role::Column::UserId))
                                    .eq(user_id);
                            let company_con =
                                Expr::col((right.clone(), rel_user_company_role::Column::CompanyId))
                                    .eq(company_id);

                            user_con.and(company_con).into_condition()
                        }),
                ).filter(condition)
                .one(conn)
                .await;
            if relation_result.is_err() || relation_result.is_ok_and(|e|e.is_none()) {
                return Err(AppError::Forbidden(403000, "".to_string()));
            }
        }
    }
    Ok("Done".to_string())
}
