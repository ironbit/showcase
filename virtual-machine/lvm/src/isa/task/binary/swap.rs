use {
    crate::{core, isa, mem},
    anyhow::Result,
    mcs::{BinaryExt, CmdExt, Make},
    std::fmt,
};

#[derive(Debug, Make, CmdExt, BinaryExt)]
pub struct Swap(core::Cmd);

impl<S> isa::BinaryExt<S> for Swap
where
    S: mem::StackExt + fmt::Debug,
{
    #[rustfmt::skip]
    fn perform(&self, lhs: core::Param, rhs: core::Param) -> Result<(isa::Status, Option<core::Param>, Option<core::Param>)> {
        Ok((isa::Status::as_running(), Some(rhs), Some(lhs)))
    }
}
