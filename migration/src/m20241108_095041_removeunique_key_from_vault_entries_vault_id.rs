use sea_orm_migration::prelude::*;

use crate::{m20241024_230139_create_vault_entry_table::VaultEntry, TableName};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(TableName("vault_entries".to_string()))
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
                            .table(TableName("vault_entries".to_string()))
                    .add_column(ColumnDef::new(Alias::new("vault_id")).uuid())
                    .to_owned(),
            )
            .await
    }
}
