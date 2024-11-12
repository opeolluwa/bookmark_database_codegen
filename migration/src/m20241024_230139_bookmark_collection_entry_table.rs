use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241024_224838_create_bookmark_collection_database::BookmarkCollection;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BookmarkCollectionEntry::Table)
                    .if_not_exists()
                    .col(uuid(BookmarkCollectionEntry::Id).unique_key().primary_key())
                    .col(string(BookmarkCollectionEntry::Title).not_null())
                    .col(string(BookmarkCollectionEntry::Description))
                    .col(uuid(BookmarkCollectionEntry::BookmarkCollectionId))
                    .col(
                        date_time(BookmarkCollectionEntry::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time(BookmarkCollectionEntry::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-BookmarkCollection-to-BookmarkCollection-entry-id")
                            .from(
                                BookmarkCollectionEntry::Table,
                                BookmarkCollectionEntry::BookmarkCollectionId,
                            )
                            .to(BookmarkCollection::Table, BookmarkCollection::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(BookmarkCollectionEntry::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum BookmarkCollectionEntry {
    Table,
    Id,
    Title,
    Description,
    BookmarkCollectionId,
    CreatedAt,
    UpdatedAt,
}
