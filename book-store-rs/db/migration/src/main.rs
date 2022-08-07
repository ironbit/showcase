// SPDX-License-Identifier: Apache-2.0
use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    cli::run_cli(db_migration::Migrator).await;
}
