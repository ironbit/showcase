use {
    crate::{isa, mem, vm},
    anyhow::Result,
    std::fmt,
};

pub struct Processor<S>
where
    S: mem::StackExt + fmt::Debug,
{
    context: vm::Context<S>,
    commands: vm::Commands<S>,
}

impl<S> Processor<S>
where
    S: mem::StackExt + fmt::Debug,
{
    pub fn new(context: vm::Context<S>, commands: vm::Commands<S>) -> Self {
        Self { context, commands }
    }

    pub fn execute(&mut self, program: vm::Program) -> Result<()> {
        self.context.reset();

        loop {
            let counter = self.context.load_counter();
            let code = program.load_code(counter)?;
            let command = self.commands.load_command(code.opcode)?;

            match command.execute(&mut self.context, &code)? {
                isa::Status::Running => (),
                isa::Status::Branch(index) => self.context.save_counter(index),
                isa::Status::Finished => break,
            }
        }

        Ok(())
    }
}

impl<S> fmt::Display for Processor<S>
where
    S: mem::StackExt + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", &self.context.stack)
    }
}

impl<S> fmt::Debug for Processor<S>
where
    S: mem::StackExt + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Processor").field("context", &self.context).finish()
    }
}
