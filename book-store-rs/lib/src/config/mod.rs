// SPDX-License-Identifier: Apache-2.0
use anyhow::Result;

pub mod database;

#[derive(Debug, Clone)]
pub struct Config {
    pub database: database::Params,
}

impl Config {
    pub fn new() -> Result<Self> {
        let db = database::Params::new()?;

        Ok(Config { database: db })
    }
}
