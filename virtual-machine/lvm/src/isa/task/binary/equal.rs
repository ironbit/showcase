use {
    crate::{
        core::{self, CmdExt as _},
        isa, mem,
    },
    anyhow::Result,
    mcs::{BinaryExt, CmdExt, Make},
    std::fmt,
};

#[derive(Debug, Make, CmdExt, BinaryExt)]
pub struct Equal(core::Cmd);

impl<S> isa::BinaryExt<S> for Equal
where
    S: mem::StackExt + fmt::Debug,
{
    #[rustfmt::skip]
    fn perform(&self, lhs: core::Param, rhs: core::Param) -> Result<(isa::Status, Option<core::Param>, Option<core::Param>)> {
        let param = if lhs == rhs {
            core::Param::new(self.rank(), 1)
        } else {
            core::Param::new(self.rank(), 0)
        };

        Ok((isa::Status::as_running(), None, Some(param)))
    }
}
