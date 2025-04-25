use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "company")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64, // Or i64 if using bigint in DB
    pub name: String,
    pub description: String,
    pub logo: Option<String>,
    pub slug: Option<String>,
    pub status: String,
    pub domain: String,
    #[sea_orm(column_name = "user_id")]
    pub user_id: i64,
    pub address: Option<String>,
    pub centra: Option<String>,
    pub city: Option<String>,
    pub email: Option<String>,
    pub industry: Option<String>,
    pub longitude: Option<String>,
    pub latitude: Option<String>,
    pub lisence_no: Option<String>,
    pub website: Option<String>,
    #[sea_orm(column_name = "company_member")]
    pub company_member: Option<String>,
    pub phone: Option<String>,
    #[sea_orm(column_name = "product_type")]
    pub product_type: Option<String>,
    pub province: Option<String>,
    #[sea_orm(column_name = "tax_id")]
    pub tax_id: Option<String>,
    pub country: Option<String>,
    #[sea_orm(column_name = "bg_image")]
    pub bg_image: Option<String>,
    #[sea_orm(column_name = "editor_id")]
    pub editor_id: Option<String>,
    #[sea_orm(column_name = "editor_name")]
    pub editor_name: Option<String>,
    #[sea_orm(column_name = "pic_first_name")]
    pub pic_first_name: Option<String>,
    #[sea_orm(column_name = "pic_last_name")]
    pub pic_last_name: Option<String>,
    #[sea_orm(column_name = "pic_contactable")]
    pub pic_contactable: Option<String>,
    #[sea_orm(column_name = "pic_phone")]
    pub pic_phone: Option<String>,
    #[sea_orm(column_name = "pic_office_phone")]
    pub pic_office_phone: Option<String>,
    #[sea_orm(column_name = "pic_email")]
    pub pic_email: Option<String>,
    #[sea_orm(column_name = "ministry_registered")]
    pub ministry_registered: Option<String>,
    #[sea_orm(column_name = "send_contract")]
    pub send_contract: Option<String>,
    #[sea_orm(column_name = "contract_doc")]
    pub contract_doc: Option<String>,
    #[sea_orm(column_name = "checked_all_data")]
    pub checked_all_data: Option<bool>,
    #[sea_orm(column_name = "created_at")]
    pub created_at: DateTime,
    #[sea_orm(column_name = "created_by")]
    pub created_by: String,
    #[sea_orm(column_name = "updated_at")]
    pub updated_at: DateTime,
    #[sea_orm(column_name = "updated_by")]
    pub updated_by: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
