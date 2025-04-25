use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .modify_column(ColumnDef::new(User::ResetKey).string().null())
                    .modify_column(ColumnDef::new(User::ResetDate).timestamp().null())
                    .drop_column(User::MustChangePassword)
                    .add_column(
                        ColumnDef::new(User::MustChangePassword)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .drop_column(User::EnforcePasswordPolicy)
                    .add_column(
                        ColumnDef::new(User::EnforcePasswordPolicy)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .drop_column(User::WrongPasswordLocked)
                    .add_column(
                        ColumnDef::new(User::WrongPasswordLocked)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .drop_column(User::Notification)
                    .add_column(
                        ColumnDef::new(User::Notification)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .modify_column(ColumnDef::new(User::LockedDate).timestamp().null())
                    .modify_column(ColumnDef::new(User::ActivatedAt).timestamp().null())
                    .modify_column(ColumnDef::new(User::EmployeeId).big_integer().null())
                    .modify_column(ColumnDef::new(User::Photo).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .modify_column(ColumnDef::new(User::ResetKey).string().null())
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    ResetDate,
    ResetKey,
    MustChangePassword,
    EnforcePasswordPolicy,
    WrongPasswordLocked,
    LockedDate,
    Notification,

    ActivatedAt,
    Photo,
    EmployeeId,
}
