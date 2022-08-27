// SPDX-License-Identifier: Apache-2.0
use {
    crate::config,
    anyhow::{Error, Result},
    sea_orm::{ConnectOptions, DatabaseConnection},
};

pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new(cfg: config::Config) -> Result<Self> {
        let url = build_url(cfg).await?;
        let opt = ConnectOptions::new(url);
        let connection = sea_orm::Database::connect(opt).await?;
        Ok(Self { connection })
    }
}

async fn build_url(cfg: config::Config) -> Result<String> {
    async {
        let mut s = String::new();
        s.push_str(&fetch_value(&cfg, config::database::Key::Kind)?);
        s.push_str("://");
        s.push_str(&fetch_value(&cfg, config::database::Key::User)?);
        s.push(':');
        s.push_str(&fetch_value(&cfg, config::database::Key::Pass)?);
        s.push('@');
        s.push_str(&fetch_value(&cfg, config::database::Key::Host)?);
        s.push(':');
        s.push_str(&fetch_value(&cfg, config::database::Key::Port)?);
        s.push('/');
        s.push_str(&fetch_value(&cfg, config::database::Key::Name)?);
        Ok(s)
    }
    .await
}

fn fetch_value(cfg: &config::Config, key: config::database::Key) -> Result<String> {
    match cfg.database.0.get(&key) {
        Some(value) => Ok(value.clone()),
        None => Err(Error::msg("")),
    }
}
