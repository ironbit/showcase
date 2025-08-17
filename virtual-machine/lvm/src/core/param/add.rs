use {super::Param, std::ops::Add};

impl Add for Param {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::R8(lhs), Self::R8(rhs)) => Some(Self::R8(lhs + rhs)),
            (Self::R16(lhs), Self::R16(rhs)) => Some(Self::R16(lhs + rhs)),
            (Self::R32(lhs), Self::R32(rhs)) => Some(Self::R32(lhs + rhs)),
            (Self::R64(lhs), Self::R64(rhs)) => Some(Self::R64(lhs + rhs)),
            (Self::R128(lhs), Self::R128(rhs)) => Some(Self::R128(lhs + rhs)),
            _ => None,
        }
    }
}
