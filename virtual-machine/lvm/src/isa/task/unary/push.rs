use {
    crate::{core, isa, mem},
    anyhow::Result,
    mcs::{CmdExt, Make, UnaryExt},
    std::fmt,
};

#[derive(Debug, Make, CmdExt, UnaryExt)]
pub struct Push(core::Cmd);

impl<S> isa::UnaryExt<S> for Push
where
    S: mem::StackExt + fmt::Debug,
{
    fn perform(&self, param: core::Param) -> Result<(isa::Status, Option<core::Param>, Option<core::Param>)> {
        Ok((isa::Status::as_running(), None, Some(param)))
    }
}
