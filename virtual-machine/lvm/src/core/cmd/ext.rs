use crate::core;

pub trait CmdExt {
    fn opcode(&self) -> core::Opcode;

    fn rank(&self) -> core::Rank;

    fn kind(&self) -> core::Kind;

    fn task(&self) -> core::Task;
}
