use {
    serde::{Deserialize, Serialize},
    std::ops::AddAssign,
};

#[derive(Copy, Clone, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Index(usize);

impl Index {
    pub fn new<T>(val: T) -> Self
    where
        T: Into<Index>,
    {
        val.into()
    }

    pub fn inner(&self) -> usize {
        self.0
    }
}

impl From<i32> for Index {
    fn from(n: i32) -> Self {
        Self(n as usize)
    }
}

impl From<usize> for Index {
    fn from(n: usize) -> Self {
        Self(n)
    }
}

impl<T> AddAssign<T> for Index
where
    T: TryInto<usize>,
{
    fn add_assign(&mut self, val: T) {
        self.0 += val.try_into().unwrap_or_default();
    }
}
