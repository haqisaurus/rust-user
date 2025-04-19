use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(UserCompanyPermission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserCompanyPermission::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(big_integer(UserCompanyPermission::UserId))
                    .col(big_integer(UserCompanyPermission::CompanyId))
                    .col(big_integer(UserCompanyPermission::PermissionId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(UserCompanyPermission::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserCompanyPermission {
    Table,
    Id,
    UserId,
    CompanyId,
    PermissionId,
}
