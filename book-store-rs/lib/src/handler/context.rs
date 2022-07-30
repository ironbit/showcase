// SPDX-License-Identifier: Apache-2.0
use crate::model::Database;

pub struct Context {
    pub database: Database,
}

impl Context {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}
