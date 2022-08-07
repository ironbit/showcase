// SPDX-License-Identifier: Apache-2.0
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Authors::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Authors::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Authors::FirstName).string().not_null())
                    .col(ColumnDef::new(Authors::LastName).string().not_null())
                    .clone(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Authors::Table).clone())
            .await
    }
}

#[derive(Iden)]
pub enum Authors {
    Table,
    Id,
    FirstName,
    LastName,
}
