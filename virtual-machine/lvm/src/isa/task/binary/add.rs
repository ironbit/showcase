use {
    crate::{core, isa, mem},
    anyhow::{Error, Result},
    mcs::{BinaryExt, CmdExt, Make},
    std::fmt,
};

#[derive(Debug, Make, CmdExt, BinaryExt)]
pub struct Add(core::Cmd);

impl<S> isa::BinaryExt<S> for Add
where
    S: mem::StackExt + fmt::Debug,
{
    #[rustfmt::skip]
    fn perform(&self, lhs: core::Param, rhs: core::Param) -> Result<(isa::Status, Option<core::Param>, Option<core::Param>)> {
        let res = (lhs + rhs).ok_or(Error::msg("perform - add"))?;
        Ok((isa::Status::as_running(), Some(res), None))
    }
}
