
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64, // Or i64 if using bigint in DB

    pub name: String,
    pub description: String,
    pub group: String,
    #[sea_orm(column_name = "created_at")]
    pub created_at: DateTime,
    #[sea_orm(column_name = "created_by")]
    pub created_by: String,
    #[sea_orm(column_name = "updated_at")]
    pub updated_at: Option<DateTime>,
    #[sea_orm(column_name = "updated_by")]
    pub updated_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    RolePermission
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::RolePermission => Entity::has_many(super::rel_role_permission::Entity).into(),
        }
    }
}

impl Related<super::permission::Entity> for Entity {
    fn to() -> RelationDef {
        super::rel_role_permission::Relation::Permission.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::rel_role_permission::Relation::Role.def().rev())
    }
}

impl ActiveModelBehavior for  ActiveModel {}