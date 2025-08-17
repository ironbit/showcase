use {
    crate::{core, isa, mem},
    anyhow::Result,
    mcs::{CmdExt, Make, NullaryExt},
    std::fmt,
};

#[derive(Debug, Make, CmdExt, NullaryExt)]
pub struct Halt(core::Cmd);

impl<S> isa::NullaryExt<S> for Halt
where
    S: mem::StackExt + fmt::Debug,
{
    fn perform(&self) -> Result<isa::Status> {
        Ok(isa::Status::as_finished())
    }
}
