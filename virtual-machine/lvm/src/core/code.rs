use {
    crate::core,
    serde::{Deserialize, Serialize},
};

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Code {
    pub opcode: core::Opcode,
    pub value: Option<core::Value>,
    pub load: Option<core::Index>,
    pub save: Option<core::Index>,
}

impl Code {
    pub fn new<T>(opcode: T) -> Self
    where
        T: Into<core::Opcode>,
    {
        Self {
            opcode: opcode.into(),
            ..Default::default()
        }
    }

    pub fn with_value(mut self, value: core::Value) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_load(mut self, index: core::Index) -> Self {
        self.load = Some(index);
        self
    }

    pub fn with_rank(mut self, index: core::Index) -> Self {
        self.save = Some(index);
        self
    }
}
