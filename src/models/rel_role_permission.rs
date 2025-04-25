use sea_orm::{ActiveModelBehavior, PrimaryKeyTrait, Related};
use sea_orm::DerivePrimaryKey;
use sea_orm::{DeriveEntityModel, EntityTrait, EnumIter, RelationDef, RelationTrait};
use serde::{Deserialize, Serialize};
use crate::models::{permission, role};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "role_permission")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64, // Or i64 if using bigint in DB

    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub permission_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Role,
    Permission,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Role => Entity::belongs_to(super::role::Entity)
                .from(Column::RoleId)
                .to(super::role::Column::Id)
                .into(),
            Self::Permission => Entity::belongs_to(super::permission::Entity)
                .from(Column::PermissionId)
                .to(super::permission::Column::Id)
                .into(),
        }
    }
}

// 👇 Add this to implement Related<Permission>
impl Related<permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permission.def()
    }
}

// 👇 Add this to implement Related<Role>
impl Related<role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
