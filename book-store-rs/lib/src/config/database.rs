// SPDX-License-Identifier: Apache-2.0
use {
    anyhow::{Error, Result},
    lazy_static::lazy_static,
    std::{collections::hash_map::HashMap, env},
};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Kind {
    Postgres,
}

lazy_static! {
    static ref KIND_WORD: HashMap<Kind, &'static str> =
        HashMap::from([(Kind::Postgres, "postgres")]);
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Key {
    Kind,
    Host,
    Port,
    Name,
    User,
    Pass,
}

lazy_static! {
    static ref KEY_WORD: HashMap<Key, &'static str> = {
        HashMap::from([
            (Key::Kind, "DB_KIND"),
            (Key::Host, "DB_HOST"),
            (Key::Port, "DB_PORT"),
            (Key::Name, "DB_NAME"),
            (Key::User, "DB_USER"),
            (Key::Pass, "DB_PASS"),
        ])
    };
}

#[derive(Debug, Default, Clone)]
pub struct Params(pub HashMap<Key, String>);

impl Params {
    pub fn new() -> Result<Self> {
        let mut map: HashMap<Key, String> = HashMap::with_capacity(KEY_WORD.len());
        for (&key, &value) in KEY_WORD.iter() {
            match env::var(value) {
                Ok(var) => map.entry(key).or_insert(var),
                Err(_) => return Err(Error::msg("")),
            };
        }

        Ok(Params(map))
    }
}
