use crate::models::{company, permission, user};
use sea_orm::DerivePrimaryKey;
use sea_orm::{ActiveModelBehavior, PrimaryKeyTrait, Related};
use sea_orm::{DeriveEntityModel, EntityTrait, EnumIter, RelationDef, RelationTrait};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_company_permission")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64, // Or i64 if using bigint in DB

    pub user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub permission_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub company_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    Permission,
    Company,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::UserId)
                .to(super::user::Column::Id)
                .into(),
            Self::Permission => Entity::belongs_to(super::permission::Entity)
                .from(Column::PermissionId)
                .to(super::permission::Column::Id)
                .into(),
            Self::Company => Entity::belongs_to(super::company::Entity)
                .from(Column::CompanyId)
                .to(super::company::Column::Id)
                .into(),
        }
    }
}

// 👇 Add this to implement Related<Company>
impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
// 👇 Add this to implement Related<Role>
impl Related<permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permission.def()
    }
}
// 👇 Add this to implement Related<Company>
impl Related<company::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Company.def()
    }
}



impl ActiveModelBehavior for ActiveModel {}
