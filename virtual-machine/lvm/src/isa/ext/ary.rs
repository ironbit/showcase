use {
    super::CommandExt,
    crate::{core, mem},
    anyhow::{Error, Result},
    std::fmt,
};

pub trait AryExt<S>: core::CmdExt + CommandExt<S>
where
    S: mem::StackExt + fmt::Debug,
{
    fn load_lhs_param(&self, stack: &mut S, code: &core::Code) -> Result<core::Param> {
        if let Some(value) = code.value {
            return Ok(core::Param::new(self.rank(), value));
        }
        if let Some(index) = code.load {
            return stack.load(index, self.rank()).ok_or(Error::msg("stack load"));
        }
        stack.pop(self.rank()).ok_or(Error::msg("stack pop"))
    }

    fn load_rhs_param(&self, stack: &mut S) -> Result<core::Param> {
        stack.pop(self.rank()).ok_or(Error::msg("stack pop"))
    }

    fn save_lhs_param(&self, stack: &mut S, code: &core::Code, param: Option<core::Param>) -> Result<()> {
        if let Some(param) = param {
            match code.save {
                None => stack.push(param),
                Some(index) => stack.save(index, param).ok_or(Error::msg("stack save"))?,
            }
        }
        Ok(())
    }

    fn save_rhs_param(&self, stack: &mut S, param: Option<core::Param>) {
        if let Some(param) = param {
            stack.push(param);
        }
    }
}
