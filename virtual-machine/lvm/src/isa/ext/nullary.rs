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

    fn process(&self, _ctx: &mut vm::Context<S>, _code: &core::Code) -> Result<Status> {
        self.perform()
    }
}
