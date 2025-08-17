use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Value(i64);

impl Value {
    pub fn new<T>(val: T) -> Self
    where
        T: Into<i64>,
    {
        Self(val.into())
    }

    pub fn inner(&self) -> i64 {
        self.0
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Self(n as i64)
    }
}
