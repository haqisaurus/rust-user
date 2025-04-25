use sea_orm_migration::{prelude::*};

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
                    .table(UserAuditLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserAuditLog::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserAuditLog::UserId).big_integer().null().default(0))
                    .col(ColumnDef::new(UserAuditLog::Username).string().not_null())
                    .col(ColumnDef::new(UserAuditLog::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(UserAuditLog::Status).string().not_null())
                    .col(ColumnDef::new(UserAuditLog::UserAgent).string().null())
                    .col(ColumnDef::new(UserAuditLog::Ip).string().null())
                    .col(ColumnDef::new(UserAuditLog::ExpiredAt).timestamp().null())
                    .col(ColumnDef::new(UserAuditLog::Token).string().null())
                    .col(ColumnDef::new(UserAuditLog::RefreshToken).string().null())
                    .col(ColumnDef::new(UserAuditLog::Platform).string().null())
                    .col(ColumnDef::new(UserAuditLog::Activity).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(UserAuditLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserAuditLog {
    Table,
    Id,
    UserId,
    Username,
    CreatedAt,
    Status,
    UserAgent,
    Ip,
    ExpiredAt,
    Token,
    RefreshToken,
    Platform,
    Activity,
}
