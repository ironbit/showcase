use {super::Param, crate::core, anyhow::Error};

impl TryFrom<Param> for core::Index {
    type Error = Error;

    fn try_from(param: Param) -> Result<Self, Self::Error> {
        let index = match param {
            Param::R8(n) => usize::try_from(n),
            Param::R16(n) => usize::try_from(n),
            Param::R32(n) => usize::try_from(n),
            Param::R64(n) => usize::try_from(n),
            Param::R128(n) => usize::try_from(n),
        };

        Ok(core::Index::new(index.map_err(Error::msg)?))
    }
}
