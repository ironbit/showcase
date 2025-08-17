use crate::core;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Status {
    Branch(core::Index),
    Running,
    Finished,
}

impl Status {
    pub fn as_branch(index: core::Index) -> Self {
        Self::Branch(index)
    }

    pub fn as_running() -> Self {
        Self::Running
    }

    pub fn as_finished() -> Self {
        Self::Finished
    }
}
