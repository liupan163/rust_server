pub use sea_orm_migration::prelude::*;

mod block_data;
mod tx_data;
mod user_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(user_data::Migration),
            Box::new(block_data::Migration),
            Box::new(tx_data::Migration),
        ]
    }

}
