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
                    .table(Company::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Company::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(string(Company::Name).not_null())
                    .col(text_null(Company::Description))
                    .col(ColumnDef::new(Company::Logo).string().null())
                    .col(ColumnDef::new(Company::Slug).string().null())
                    .col(ColumnDef::new(Company::Status).string().not_null())
                    .col(ColumnDef::new(Company::UserID).big_integer().not_null())
                    .col(ColumnDef::new(Company::Address).text().null())
                    .col(ColumnDef::new(Company::Centra).string().null())
                    .col(ColumnDef::new(Company::City).string().null())
                    .col(ColumnDef::new(Company::Email).string().null())
                    .col(ColumnDef::new(Company::Industry).string().null())
                    .col(ColumnDef::new(Company::Longitude).string().null())
                    .col(ColumnDef::new(Company::Latitude).string().null())
                    .col(ColumnDef::new(Company::LisenceNo).string().null())
                    .col(ColumnDef::new(Company::Website).string().null())
                    .col(ColumnDef::new(Company::CompanyMember).string().null())
                    .col(ColumnDef::new(Company::Phone).string().null())
                    .col(ColumnDef::new(Company::ProductType).string().null())
                    .col(ColumnDef::new(Company::Province).string().null())
                    .col(ColumnDef::new(Company::TaxID).string().null())
                    .col(ColumnDef::new(Company::Country).string().null())
                    .col(ColumnDef::new(Company::BgImage).string().null())
                    .col(ColumnDef::new(Company::EditorID).string().null())
                    .col(ColumnDef::new(Company::EditorName).string().null())
                    .col(ColumnDef::new(Company::PICFirstName).string().null())
                    .col(ColumnDef::new(Company::PICLastName).string().null())
                    .col(ColumnDef::new(Company::PICContactable).string().null())
                    .col(ColumnDef::new(Company::PICPhone).string().null())
                    .col(ColumnDef::new(Company::PICOfficePhone).string().null())
                    .col(ColumnDef::new(Company::PICEmail).string().null())
                    .col(ColumnDef::new(Company::MinistryRegistered).string().null())
                    .col(ColumnDef::new(Company::SendContract).string().null())
                    .col(ColumnDef::new(Company::ContractDoc).string().null())
                    .col(ColumnDef::new(Company::CheckedAllData).boolean().default(false))
                    .col(timestamp(Company::CreatedAt))
                    .col(string(Company::CreatedBy))
                    .col(timestamp_null(Company::UpdatedAt))
                    .col(ColumnDef::new(Company::UpdatedBy).string().null())
                    .col(timestamp_null(Company::DeletedAt))
                    .col(ColumnDef::new(Company::DeletedBy).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Company::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Company {
    Table,
    Id,
    Name,
    Description,
    Logo,
    Slug,
    Status,
    UserID,
    Address,
    Centra,
    City,
    Email,
    Industry,
    Latitude,
    LisenceNo,
    Longitude,
    Website,
    CompanyMember,
    Phone,
    ProductType,
    Province,
    TaxID,
    Country,
    BgImage,
    EditorID,
    EditorName,
    PICFirstName,
    PICLastName,
    PICContactable,
    PICPhone,
    PICOfficePhone,
    PICEmail,
    MinistryRegistered,
    SendContract,
    ContractDoc,
    CheckedAllData,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy
}
