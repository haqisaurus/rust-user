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
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(string(User::Username))
                    .col(string(User::Password))
                    .col(string(User::FirstName))
                    .col(string(User::LastName))
                    .col(string_uniq(User::Email))
                    .col(boolean(User::Activated))
                    .col(string(User::Language))
                    .col(string(User::Currency))
                    .col(string(User::Notification))
                    .col(string(User::ActivationKey))
                    .col(string(User::ResetKey))
                    .col(timestamp_null(User::ResetDate))
                    .col(boolean(User::Admin))
                    .col(string(User::MustChangePassword))
                    .col(string(User::EnforcePasswordPolicy))
                    .col(string(User::WrongPasswordLocked))
                    .col(timestamp_null(User::LockedDate))
                    .col(boolean(User::DisableMobileAndroid))
                    .col(boolean(User::DisableMobileIos))
                    .col(boolean(User::DisableWeb))
                    .col(string(User::AccountType))
                    .col(timestamp(User::CreatedAt))
                    .col(string(User::CreatedBy))
                    .col(timestamp_null(User::UpdatedAt))
                    .col(ColumnDef::new(User::UpdatedBy).string().null())
                    .col(integer(User::EmployeeId))
                    .col(timestamp_null(User::ActivatedAt))
                    .col(string(User::Photo))
                    .col(timestamp_null(User::DeletedAt))
                    .col(ColumnDef::new(User::DeletedBy).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Password,
    FirstName,
    LastName,
    Email,
    Activated,
    Language,
    Currency,
    Notification,
    ActivationKey,
    ResetDate,
    ResetKey,
    Admin,
    MustChangePassword,
    EnforcePasswordPolicy,
    WrongPasswordLocked,
    LockedDate,
    DisableMobileAndroid,
    DisableMobileIos,
    DisableWeb,
    AccountType,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    ActivatedAt,
    Photo,
    EmployeeId,
    DeletedAt,
    DeletedBy
}
