use super::Param;

impl Param {
    pub fn is_zero(&self) -> bool {
        match self {
            Self::R8(n) => *n == 0i8,
            Self::R16(n) => *n == 0i16,
            Self::R32(n) => *n == 0i32,
            Self::R64(n) => *n == 0i64,
            Self::R128(n) => *n == 0i128,
        }
    }
}
