use {
    crate::{core, isa, mem},
    anyhow::Result,
    mcs::{BinaryExt, CmdExt, Make},
    std::fmt,
};

#[derive(Debug, Make, CmdExt, BinaryExt)]
pub struct Jump(core::Cmd);

impl<S> isa::BinaryExt<S> for Jump
where
    S: mem::StackExt + fmt::Debug,
{
    #[rustfmt::skip]
    fn perform(&self, lhs: core::Param, rhs: core::Param) -> Result<(isa::Status, Option<core::Param>, Option<core::Param>)> {
        let status = if rhs.is_zero() {
            isa::Status::as_branch(lhs.try_into()?)
        } else {
            isa::Status::as_running()
        };

        Ok((status, None, None))
    }
}
