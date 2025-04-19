pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250419_083500_update_user_table;
mod m20250419_090733_create_permission_table;
mod m20250419_090944_create_role_table;
mod m20250419_092048_create_company_table;
mod m20250419_094444_create_role_permission_table;
mod m20250419_094627_create_user_company_role_table;
mod m20250419_094816_create_user_company_permission_table;
mod m20250419_095857_create_user_audit_log_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250419_083500_update_user_table::Migration),
            Box::new(m20250419_090733_create_permission_table::Migration),
            Box::new(m20250419_090944_create_role_table::Migration),
            Box::new(m20250419_092048_create_company_table::Migration),
            Box::new(m20250419_094444_create_role_permission_table::Migration),
            Box::new(m20250419_094627_create_user_company_role_table::Migration),
            Box::new(m20250419_094816_create_user_company_permission_table::Migration),
            Box::new(m20250419_095857_create_user_audit_log_table::Migration),
        ]
    }
}
