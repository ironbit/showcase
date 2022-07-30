// SPDX-License-Identifier: Apache-2.0
use {
    anyhow::Result,
    sea_orm::{ConnectOptions, DatabaseConnection},
};

pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let opt = ConnectOptions::new(
            "postgres://postgres:postgres@localhost:5432/book_store_db".to_owned(),
        );
        let connection = sea_orm::Database::connect(opt).await?;
        Ok(Self { connection })
    }
}
