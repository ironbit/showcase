use {
    crate::core,
    anyhow::{Error, Result},
};

#[derive(Clone, Debug)]
pub struct Program {
    inner: Vec<core::Code>,
}

impl Program {
    pub fn new(codes: Vec<core::Code>) -> Self {
        Self { inner: codes }
    }

    pub fn load_code(&self, index: core::Index) -> Result<core::Code> {
        let i = index.inner();

        if self.inner.len() <= i {
            return Err(Error::msg("index overflow"));
        }

        Ok(self.inner[i])
    }
}
