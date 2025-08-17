use {
    crate::core,
    serde::{Deserialize, Serialize},
};

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Cmd {
    pub opcode: core::Opcode,
    pub kind: core::Kind,
    pub task: core::Task,
    #[serde(default)]
    pub rank: core::Rank,
}

impl Cmd {
    pub fn new<T>(opcode: T) -> Self
    where
        T: Into<core::Opcode>,
    {
        Self {
            opcode: opcode.into(),
            ..Default::default()
        }
    }

    pub fn with_kind(mut self, kind: core::Kind) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_task(mut self, task: core::Task) -> Self {
        self.task = task;
        self
    }

    pub fn with_rank(mut self, rank: core::Rank) -> Self {
        self.rank = rank;
        self
    }
}
