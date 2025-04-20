use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_audit_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64, // Or i64 if using bigint in DB
    #[sea_orm(column_name = "user_id")]
    pub user_id: i64,
    #[sea_orm(column_name = "username")]
    pub username: String,
    #[sea_orm(column_name = "created_at")]
    pub created_at: DateTime,
    #[sea_orm(column_name = "status")]
    pub status: String,
    #[sea_orm(column_name = "user_agent")]
    pub user_agent: String,
    #[sea_orm(column_name = "ip")]
    pub ip: String,
    #[sea_orm(column_name = "expired_at")]
    pub expired_at: Option<DateTime>,
    #[sea_orm(column_name = "token")]
    pub token: Option<String>,
    #[sea_orm(column_name = "refresh_token")]
    pub refresh_token: Option<String>,
    #[sea_orm(column_name = "platform")]
    pub platform: String,
    #[sea_orm(column_name = "activity")]
    pub activity: String,
}


#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    // #[sea_orm(
    //     belongs_to = "super::cake::Entity",
    //     from = "Column::CakeId",
    //     to = "super::cake::Column::Id"
    // )]
    User,
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::Username)
                .to(super::user::Column::Username)
                .into(),
        }
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
