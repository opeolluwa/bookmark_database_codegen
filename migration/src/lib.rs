pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20241024_224838_create_vaults_database;
mod m20241024_230139_create_vault_entry_table;
mod m20241030_121715_rename_vault_entry_table;
mod m20241108_095041_removeunique_key_from_vault_entries_vault_id;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241024_224838_create_vaults_database::Migration),
            Box::new(m20241024_230139_create_vault_entry_table::Migration),
            Box::new(m20241030_121715_rename_vault_entry_table::Migration),
            Box::new(m20241108_095041_removeunique_key_from_vault_entries_vault_id::Migration),
        ]
    }
}
