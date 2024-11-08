use sea_orm_migration::prelude::*;

use crate::m20241024_230139_create_vault_entry_table::VaultEntry;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(VaultEntry::Table)
                    .drop_column(VaultEntry::VaultId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(VaultEntry::Table)
                    .add_column(ColumnDef::new(Alias::new("vault_id")).uuid())
                    .to_owned(),
            )
            .await
    }
}
