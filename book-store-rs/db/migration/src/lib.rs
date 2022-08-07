// SPDX-License-Identifier: Apache-2.0
pub use sea_orm_migration::prelude::*;

mod m20220801_231729_category;
mod m20220801_231937_authors;
mod m20220801_232341_books;
mod m20220801_232913_authors_books;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220801_231729_category::Migration),
            Box::new(m20220801_231937_authors::Migration),
            Box::new(m20220801_232341_books::Migration),
            Box::new(m20220801_232913_authors_books::Migration),
        ]
    }
}
