use sea_orm_migration::prelude::*;
use sea_orm::DeriveIden;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserInfo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserInfo::Name).text().not_null())
                    .col(ColumnDef::new(UserInfo::Age).integer().not_null())
                    .col(ColumnDef::new(UserInfo::Image).text().not_null())
                    .col(ColumnDef::new(UserInfo::Email).text().not_null())
                    .col(ColumnDef::new(UserInfo::Password).text().not_null())
                    .col(ColumnDef::new(UserInfo::WalletAddress).text().not_null())
                    .col(ColumnDef::new(UserInfo::CreatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(UserInfo::UpdatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserInfo {
    Table,
    Id,
    Name,
    Age,
    Image,
    Email,
    Password,
    WalletAddress,
    CreatedAt,
    UpdatedAt,
}
