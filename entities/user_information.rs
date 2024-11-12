//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_information")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::bookmark_collection::Entity")]
    BookmarkCollection,
}

impl Related<super::bookmark_collection::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookmarkCollection.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
