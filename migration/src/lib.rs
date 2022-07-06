pub use sea_orm_migration::prelude::*;

mod m20220704_165625_create_player_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220704_165625_create_player_table::Migration),
        ]
    }
}
