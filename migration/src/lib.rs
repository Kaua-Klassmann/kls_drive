pub use sea_orm_migration::prelude::*;

mod m20250304_131008_create_user_table;
mod m20250304_131553_create_document_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250304_131008_create_user_table::Migration),
            Box::new(m20250304_131553_create_document_table::Migration),
        ]
    }
}
