use {
    super::{Status, ary::AryExt},
    crate::{core, mem, vm},
    anyhow::Result,
    std::fmt,
};

pub trait UnaryExt<S>: AryExt<S>
where
    S: mem::StackExt + fmt::Debug,
{
    fn perform(&self, param: core::Param) -> Result<(Status, Option<core::Param>, Option<core::Param>)>;

    fn process(&self, code: &core::Code, ctx: &mut vm::Context<S>) -> Result<Status> {
        let stack = ctx.stack();

        let param = self.load_lhs_param(stack, code)?;

        let (status, lhs, rhs) = self.perform(param)?;

        self.save_lhs_param(stack, code, lhs)?;
        self.save_rhs_param(stack, rhs);

        Ok(status)
    }
}
