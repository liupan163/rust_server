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
                    .table(TxInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TxInfo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TxInfo::BlockId).integer().not_null())
                    .col(ColumnDef::new(TxInfo::TxHash).text().not_null())
                    .col(ColumnDef::new(TxInfo::TxType).integer().not_null())
                    .col(ColumnDef::new(TxInfo::TxStatus).text().not_null())
                    .col(ColumnDef::new(TxInfo::TxAmount).integer().not_null())
                    .col(ColumnDef::new(TxInfo::TxFee).integer().not_null())
                    .col(ColumnDef::new(TxInfo::TxTime).text().not_null())
                    .col(ColumnDef::new(TxInfo::FromAddress).text().not_null())
                    .col(ColumnDef::new(TxInfo::ToAddress).text().not_null())
                    .col(ColumnDef::new(TxInfo::TxMemo).text().not_null())
                    .col(ColumnDef::new(TxInfo::CreatedAt).date_time().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(TxInfo::UpdatedAt).date_time().not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TxInfo::Table).to_owned())      
            .await
    }
}

#[derive(DeriveIden)]
pub enum TxInfo {
    Table,
    Id,
    BlockId,
    TxHash,
    TxType,
    TxStatus,
    TxAmount,
    TxFee,
    TxTime,
    FromAddress,
    ToAddress,
    TxMemo,
    CreatedAt,
    UpdatedAt,
}
