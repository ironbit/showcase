use crate::core;

pub trait StackExt {
    fn reset(&mut self);

    fn push(&mut self, key: core::Param);

    fn pop(&mut self, rank: core::Rank) -> Option<core::Param>;

    fn save(&mut self, index: core::Index, param: core::Param) -> Option<()>;

    fn load(&self, index: core::Index, rank: core::Rank) -> Option<core::Param>;
}
