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
                    .table(UserCompanyRole::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserCompanyRole::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(big_integer(UserCompanyRole::UserId))
                    .col(big_integer(UserCompanyRole::CompanyId))
                    .col(big_integer(UserCompanyRole::RoleId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(UserCompanyRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserCompanyRole {
    Table,
    Id,
    UserId,
    CompanyId,
    RoleId,
}
