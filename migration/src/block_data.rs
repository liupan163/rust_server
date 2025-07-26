use sea_orm_migration::prelude::*;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BlockInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BlockInfo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BlockInfo::ChainId).text().not_null())
                    .col(ColumnDef::new(BlockInfo::BlockNumber).integer().not_null())
                    .col(ColumnDef::new(BlockInfo::BlockSlot).integer().not_null())
                    .col(ColumnDef::new(BlockInfo::BlockHash).text().not_null())
                    .col(ColumnDef::new(BlockInfo::BlockTime).integer().not_null())
                    .col(ColumnDef::new(BlockInfo::BlockAddress).text().not_null())
                    .col(ColumnDef::new(BlockInfo::BlockMemo).text().not_null())
                    .col(ColumnDef::new(BlockInfo::ParentHash).text().not_null())
                    .col(ColumnDef::new(BlockInfo::Nonce).integer().not_null())
                    .col(ColumnDef::new(BlockInfo::Difficulty).integer().not_null())
                    .col(
                        ColumnDef::new(BlockInfo::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(BlockInfo::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BlockInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum BlockInfo {
    Table,
    Id,
    ChainId,
    BlockNumber,
    BlockSlot,
    BlockHash,
    BlockTime,
    BlockAddress,
    BlockMemo,
    CreatedAt,
    UpdatedAt,
    ParentHash,
    Nonce,
    Difficulty,
}
