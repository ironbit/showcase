// SPDX-License-Identifier: Apache-2.0
use {
    crate::{m20220801_231937_authors::Authors, m20220801_232341_books::Books},
    sea_orm_migration::prelude::*,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuthorsBooks::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AuthorsBooks::AuthorId).uuid().not_null())
                    .col(ColumnDef::new(AuthorsBooks::BookId).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .col(AuthorsBooks::AuthorId)
                            .col(AuthorsBooks::BookId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_author")
                            .from(AuthorsBooks::Table, AuthorsBooks::AuthorId)
                            .to(Authors::Table, Authors::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_book")
                            .from(AuthorsBooks::Table, AuthorsBooks::BookId)
                            .to(Books::Table, Books::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .clone(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuthorsBooks::Table).clone())
            .await
    }
}

#[derive(Iden)]
pub enum AuthorsBooks {
    Table,
    BookId,
    AuthorId,
}
