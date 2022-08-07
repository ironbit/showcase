// SPDX-License-Identifier: Apache-2.0
use {crate::m20220801_231729_category::Category, sea_orm_migration::prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Books::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Books::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Books::Title).string().not_null())
                    .col(ColumnDef::new(Books::Category).integer().not_null())
                    .col(ColumnDef::new(Books::Edition).integer().not_null())
                    .col(ColumnDef::new(Books::Published).integer().not_null())
                    .col(ColumnDef::new(Books::Publisher).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_category")
                            .from(Books::Table, Books::Category)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .clone(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Books::Table).clone())
            .await
    }
}

#[derive(Iden)]
pub enum Books {
    Table,
    Id,
    Title,
    Category,
    Edition,
    Published,
    Publisher,
}
