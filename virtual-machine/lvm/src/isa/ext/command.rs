use {
    super::Status,
    crate::{core, mem, vm},
    anyhow::Result,
    std::fmt,
};

pub trait CommandExt<S>: core::CmdExt + fmt::Debug
where
    S: mem::StackExt + fmt::Debug,
{
    fn execute(&self, code: &core::Code, ctx: &mut vm::Context<S>) -> Result<Status>;
}
