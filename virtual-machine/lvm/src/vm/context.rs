use {
    crate::{core, mem},
    std::fmt,
};

#[derive(Debug)]
pub struct Context<S>
where
    S: mem::StackExt + fmt::Debug,
{
    pub(crate) stack: S,
    counter: core::Index,
}

impl<S> Context<S>
where
    S: mem::StackExt + fmt::Debug,
{
    pub fn new(stack: S) -> Self {
        let counter = core::Index::default();

        Self { stack, counter }
    }

    pub fn reset(&mut self) {
        self.stack.reset();
    }

    pub fn stack(&mut self) -> &mut S {
        &mut self.stack
    }

    pub fn load_counter(&mut self) -> core::Index {
        let counter = self.counter;
        self.counter += 1;
        counter
    }

    pub fn save_counter(&mut self, index: core::Index) {
        self.counter = index;
    }
}
