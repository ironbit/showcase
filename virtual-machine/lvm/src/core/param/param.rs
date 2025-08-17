use {
    crate::core,
    serde::{Deserialize, Serialize},
};

#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Param {
    R8(i8),
    R16(i16),
    R32(i32),
    R64(i64),
    R128(i128),
}

impl Param {
    #[allow(clippy::unnecessary_cast)]
    pub fn new<T>(rank: core::Rank, value: T) -> Self
    where
        T: Into<core::Value>,
    {
        let value = value.into();

        match rank {
            core::Rank::R8 => Self::with_r8(value.inner() as i8),
            core::Rank::R16 => Self::with_r16(value.inner() as i16),
            core::Rank::R32 => Self::with_r32(value.inner() as i32),
            core::Rank::R64 => Self::with_r64(value.inner() as i64),
            core::Rank::R128 => Self::with_r128(value.inner() as i128),
        }
    }

    pub fn with_r8(val: i8) -> Self {
        Self::R8(val)
    }

    pub fn with_r16(val: i16) -> Self {
        Self::R16(val)
    }

    pub fn with_r32(val: i32) -> Self {
        Self::R32(val)
    }

    pub fn with_r64(val: i64) -> Self {
        Self::R64(val)
    }

    pub fn with_r128(val: i128) -> Self {
        Self::R128(val)
    }
}

impl Param {
    pub fn load_rank(&self) -> core::Rank {
        match self {
            Self::R8(_) => core::Rank::R8,
            Self::R16(_) => core::Rank::R16,
            Self::R32(_) => core::Rank::R32,
            Self::R64(_) => core::Rank::R64,
            Self::R128(_) => core::Rank::R128,
        }
    }
}
