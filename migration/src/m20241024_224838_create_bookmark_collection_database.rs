use super::m20220101_000001_create_table::UserInformation;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BookmarkCollection::Table)
                    .if_not_exists()
                    .col(
                        uuid(BookmarkCollection::Id)
                            .unique_key()
                            .primary_key()
                            .not_null(),
                    )
                    .col(string(BookmarkCollection::Name).not_null())
                    .col(string(BookmarkCollection::Description))
                    .col(uuid(BookmarkCollection::UserId).not_null())
                    .col(date_time(BookmarkCollection::CreatedAt).default(Expr::current_date()))
                    .col(date_time(BookmarkCollection::UpdatedAt).default(Expr::current_date()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-bookmark-collection-id")
                            .from(BookmarkCollection::Table, BookmarkCollection::UserId)
                            .to(UserInformation::Table, UserInformation::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BookmarkCollection::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum BookmarkCollection {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
    UserId,
}
