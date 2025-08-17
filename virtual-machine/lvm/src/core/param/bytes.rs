use {super::Param, crate::core};

impl Param {
    pub fn to_bytes(self) -> ([u8; 16], core::Rank) {
        match self {
            Self::R8(n) => ((n as i128).to_le_bytes(), core::Rank::R8),
            Self::R16(n) => ((n as i128).to_le_bytes(), core::Rank::R16),
            Self::R32(n) => ((n as i128).to_le_bytes(), core::Rank::R32),
            Self::R64(n) => ((n as i128).to_le_bytes(), core::Rank::R64),
            Self::R128(n) => (n.to_le_bytes(), core::Rank::R128),
        }
    }

    pub fn from_bytes(data: &[u8], rank: core::Rank) -> Self {
        match rank {
            core::Rank::R8 => {
                let mut buffer = [0u8; size_of::<i8>()];
                buffer.copy_from_slice(data);
                Self::R8(i8::from_le_bytes(buffer))
            }
            core::Rank::R16 => {
                let mut buffer = [0u8; size_of::<i16>()];
                buffer.copy_from_slice(data);
                Self::R16(i16::from_le_bytes(buffer))
            }
            core::Rank::R32 => {
                let mut buffer = [0u8; size_of::<i32>()];
                buffer.copy_from_slice(data);
                Self::R32(i32::from_le_bytes(buffer))
            }
            core::Rank::R64 => {
                let mut buffer = [0u8; size_of::<i64>()];
                buffer.copy_from_slice(data);
                Self::R64(i64::from_le_bytes(buffer))
            }
            core::Rank::R128 => {
                let mut buffer = [0u8; size_of::<i128>()];
                buffer.copy_from_slice(data);
                Self::R128(i128::from_le_bytes(buffer))
            }
        }
    }
}
