use {
    super::{Status, ary::AryExt},
    crate::{core, mem, vm},
    anyhow::Result,
    std::fmt,
};

pub trait BinaryExt<S>: AryExt<S>
where
    S: mem::StackExt + fmt::Debug,
{
    #[rustfmt::skip]
    fn perform(&self, lhs: core::Param, rhs: core::Param) -> Result<(Status, Option<core::Param>, Option<core::Param>)>;

    fn process(&self, code: &core::Code, ctx: &mut vm::Context<S>) -> Result<Status> {
        let stack = ctx.stack();

        let lhs = self.load_lhs_param(stack, code)?;
        let rhs = self.load_rhs_param(stack)?;

        let (status, lhs, rhs) = self.perform(lhs, rhs)?;

        self.save_lhs_param(stack, code, lhs)?;
        self.save_rhs_param(stack, rhs);

        Ok(status)
    }
}
