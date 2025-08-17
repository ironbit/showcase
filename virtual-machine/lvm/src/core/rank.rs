use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Rank {
    R8,
    R16,
    #[default]
    R32,
    R64,
    R128,
}

impl Rank {
    pub const fn size(self) -> usize {
        match self {
            Self::R8 => size_of::<u8>(),
            Self::R16 => size_of::<u16>(),
            Self::R32 => size_of::<u32>(),
            Self::R64 => size_of::<u64>(),
            Self::R128 => size_of::<u128>(),
        }
    }
}
