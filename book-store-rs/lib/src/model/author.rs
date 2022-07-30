// SPDX-License-Identifier: Apache-2.0
use {
    super::Database,
    anyhow::{Error, Result},
    sea_orm::{entity::prelude::*, EntityTrait},
    uuid::Uuid,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "authors")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn fetch(db: &Database, id: Uuid) -> Result<Model> {
    let data = Entity::find_by_id(id).one(&db.connection).await?;
    match data {
        Some(author) => Ok(author),
        None => Err(Error::msg("")),
    }
}
