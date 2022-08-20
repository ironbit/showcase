//! `SeaORM` Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "authors_books")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub author_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub book_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::authors::Entity",
        from = "Column::AuthorId",
        to = "super::authors::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Authors,
    #[sea_orm(
        belongs_to = "super::books::Entity",
        from = "Column::BookId",
        to = "super::books::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Books,
}

impl Related<super::authors::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Authors.def()
    }
}

impl Related<super::books::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Books.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
