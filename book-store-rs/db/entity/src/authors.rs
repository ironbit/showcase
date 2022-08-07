//! `SeaORM` Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "authors")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::authors_books::Entity")]
    AuthorsBooks,
}

impl Related<super::authors_books::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthorsBooks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}