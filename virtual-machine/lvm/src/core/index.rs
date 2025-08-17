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
        T: Into<usize>,
    {
        Self(val.into())
    }

    pub fn inner(&self) -> usize {
        self.0
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
