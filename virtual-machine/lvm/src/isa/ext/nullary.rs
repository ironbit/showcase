use {
    super::{CommandExt, Status},
    crate::{core, mem, vm},
    anyhow::Result,
    std::fmt,
};

pub trait NullaryExt<S>: core::CmdExt + CommandExt<S>
where
    S: mem::StackExt + fmt::Debug,
{
    fn perform(&self) -> Result<Status>;

    fn process(&self, _code: &core::Code, _ctx: &mut vm::Context<S>) -> Result<Status> {
        self.perform()
    }
}
