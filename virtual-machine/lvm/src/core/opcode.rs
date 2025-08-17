use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Opcode(u16);

impl Opcode {
    pub fn new<T>(val: T) -> Self
    where
        T: Into<u16>,
    {
        Self(val.into())
    }

    pub fn inner(&self) -> u16 {
        self.0
    }
}

impl From<i32> for Opcode {
    fn from(val: i32) -> Self {
        Self(val as u16)
    }
}
