use crate::dto::error_dto::AppError;
use crate::models::company::Model;
use crate::models::prelude::Company;
use crate::models::company;
use sea_orm::prelude::Expr;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

pub async fn get_company_by_domain(
    domain: &String,
    conn: &DatabaseConnection,
) -> Result<Option<Model>, AppError> {
    let condition = Expr::col(company::Column::Domain).eq(domain.clone());
    let result_model = Company::find().filter(condition).one(conn).await;
    if result_model.is_err() {
        let error = result_model.unwrap_err();
        return Err(AppError::DbError(error, "".to_string()));
    }

    Ok(result_model?)
}
