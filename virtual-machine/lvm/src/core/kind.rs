use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    #[default]
    Nullary,
    Unary,
    Binary,
}

impl Kind {
    pub fn as_nullary() -> Self {
        Self::Nullary
    }

    pub fn as_unary() -> Self {
        Self::Unary
    }

    pub fn as_binary() -> Self {
        Self::Binary
    }
}
