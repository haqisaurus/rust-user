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
                    .table(Role::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Role::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(string(Role::Name).not_null())
                    .col(string(Role::Group).not_null())
                    .col(text(Role::Description).not_null())
                    .col(timestamp(Role::CreatedAt))
                    .col(string(Role::CreatedBy))
                    .col(timestamp_null(Role::UpdatedAt))
                    .col(ColumnDef::new(Role::UpdatedBy).string().null())
                    .col(timestamp_null(Role::DeletedAt))
                    .col(ColumnDef::new(Role::DeletedBy).string().null())
                    .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Role {
    Table,
    Id,
    Name,
    Group,
    Description,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy
}
