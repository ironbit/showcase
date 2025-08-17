use {
    crate::{core, isa, mem},
    anyhow::{Error, Result},
    std::{collections::HashMap, fmt, rc::Rc},
};

#[derive(Clone, Debug)]
pub struct Commands<S>
where
    S: mem::StackExt + fmt::Debug,
{
    inner: HashMap<core::Opcode, Rc<dyn isa::CommandExt<S>>>,
}

impl<S> Commands<S>
where
    S: mem::StackExt + fmt::Debug,
{
    pub fn new(commands: Vec<Rc<dyn isa::CommandExt<S>>>) -> Self {
        let inner = commands.into_iter().map(|c| (c.opcode(), c)).collect();
        Self { inner }
    }

    pub fn load_command(&self, opcode: core::Opcode) -> Result<Rc<dyn isa::CommandExt<S>>> {
        self.inner.get(&opcode).cloned().ok_or(Error::msg("opcode not found"))
    }
}
