use {
    crate::{core, isa, mem, vm},
    anyhow::{Error, Result},
    std::{default::Default, fmt, rc::Rc},
};

#[derive(Default)]
pub struct Factory<S>
where
    S: mem::StackExt + fmt::Debug + Default,
{
    inner: Vec<Rc<dyn isa::CommandExt<S>>>,
}

impl<S> Factory<S>
where
    S: mem::StackExt + fmt::Debug + Default,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_codes(mut self, cmds: Vec<core::Cmd>) -> Result<Self> {
        use core::Kind;

        self.inner = cmds
            .into_iter()
            .map(|cmd| match cmd.kind {
                Kind::Nullary => Self::load_nullary(cmd),
                Kind::Unary => Self::load_unary(cmd),
                Kind::Binary => Self::load_binary(cmd),
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(self)
    }

    pub fn make(self) -> vm::Commands<S> {
        vm::Commands::new(self.inner)
    }
}

impl<S> Factory<S>
where
    S: mem::StackExt + fmt::Debug + Default,
{
    fn load_nullary(cmd: core::Cmd) -> Result<Rc<dyn isa::CommandExt<S>>> {
        use isa::task::nullary;

        match cmd.task {
            core::Task::Halt => Ok(nullary::Halt::new(cmd)),
            _ => Err(Error::msg(format!("factory: 'cmd' not found {cmd:?}"))),
        }
    }

    fn load_unary(cmd: core::Cmd) -> Result<Rc<dyn isa::CommandExt<S>>> {
        use isa::task::unary;

        let command = match cmd.task {
            core::Task::Dupe => unary::Dupe::new(cmd),
            core::Task::Pop => unary::Pop::new(cmd),
            core::Task::Push => unary::Push::new(cmd),
            _ => return Err(Error::msg(format!("factory: 'cmd' not found {cmd:?}"))),
        };

        Ok(command)
    }

    fn load_binary(cmd: core::Cmd) -> Result<Rc<dyn isa::CommandExt<S>>> {
        use isa::task::binary;

        let command = match cmd.task {
            core::Task::Add => binary::Add::new(cmd),
            core::Task::Equal => binary::Equal::new(cmd),
            core::Task::Jump => binary::Jump::new(cmd),
            core::Task::Swap => binary::Swap::new(cmd),
            _ => return Err(Error::msg(format!("factory: 'cmd' not found {cmd:?}"))),
        };

        Ok(command)
    }
}
