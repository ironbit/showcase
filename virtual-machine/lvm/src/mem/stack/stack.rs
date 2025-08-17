use crate::{core, mem};

#[derive(Debug, Default)]
pub struct Stack {
    inner: Vec<u8>,
}

impl Stack {
    pub fn new() -> Self {
        Self::default()
    }
}

impl mem::StackExt for Stack {
    fn reset(&mut self) {
        self.inner.clear();
    }

    fn push(&mut self, param: core::Param) {
        let (array, rank) = param.to_bytes();
        self.inner.extend(&array[..rank.size()]);
    }

    fn pop(&mut self, rank: core::Rank) -> Option<core::Param> {
        let r = rank.size();
        let s = self.inner.len();
        if s < r {
            return None;
        }

        let data = self.inner.drain((s - r)..);
        Some(core::Param::from_bytes(data.as_slice(), rank))
    }

    fn save(&mut self, index: core::Index, param: core::Param) -> Option<()> {
        let rank = param.load_rank();
        self.check_index(index, rank)?;
        self.save_param(index, param);
        Some(())
    }

    fn load(&self, index: core::Index, rank: core::Rank) -> Option<core::Param> {
        self.check_index(index, rank)?;
        self.load_param(index, rank).into()
    }
}

impl Stack {
    fn check_index(&self, index: core::Index, rank: core::Rank) -> Option<()> {
        let i = index.inner() * rank.size();

        if self.inner.len() <= i {
            return None;
        }

        if self.inner.len() < (i + rank.size()) {
            return None;
        }

        Some(())
    }

    fn save_param(&mut self, index: core::Index, param: core::Param) {
        let (data, rank) = param.to_bytes();

        let i = (self.inner.len() - rank.size()) - (index.inner() * rank.size());
        let stack = &mut self.inner[i..i + rank.size()];

        stack.copy_from_slice(&data[..rank.size()]);
    }

    fn load_param(&self, index: core::Index, rank: core::Rank) -> core::Param {
        let i = (self.inner.len() - rank.size()) - (index.inner() * rank.size());

        let stack = &self.inner[i..i + rank.size()];

        core::Param::from_bytes(stack, rank)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{core, mem::StackExt},
    };

    #[test]
    fn stack_reset() {
        // Parameters
        let mut init = vec![0x7E, 0x7D];

        // Make 'stack'
        let mut stack = Stack::new();
        stack.inner.append(&mut init);

        // Verify results
        assert!(!stack.inner.is_empty());

        // Perform test
        stack.reset();

        // Verify results
        assert!(stack.inner.is_empty());
    }

    #[test]
    fn stack_push() {
        // Parameters
        let params = [
            core::Param::with_r8(0x7F),
            core::Param::with_r16(0x7D7E),
            core::Param::with_r32(0x797A7B7C),
            core::Param::with_r64(0x7172737475767778),
            core::Param::with_r128(0x6162636465666768696A6B6C6D6E6F70),
        ];

        #[rustfmt::skip]
        let expected = vec![
            0x7F, 0x7E, 0x7D, 0x7C, 0x7B,
            0x7A, 0x79, 0x78, 0x77, 0x76,
            0x75, 0x74, 0x73, 0x72, 0x71,
            0x70, 0x6F, 0x6E, 0x6D, 0x6C, 
            0x6B, 0x6A, 0x69, 0x68, 0x67,
            0x66, 0x65, 0x64, 0x63, 0x62,
            0x61,
        ];

        // Make 'stack'
        let mut stack = Stack::new();

        // Perform test
        for param in params {
            stack.push(param);
        }

        // Verify results
        let obtained = &stack.inner;
        assert_eq!(&expected, obtained)
    }

    #[test]
    fn stack_pop_param() {
        // Parameters
        #[rustfmt::skip]
        let mut init = vec![
            0x7F, 0x7E, 0x7D, 0x7C, 0x7B,
            0x7A, 0x79, 0x78, 0x77, 0x76,
            0x75, 0x74, 0x73, 0x72, 0x71,
            0x70, 0x6F, 0x6E, 0x6D, 0x6C, 
            0x6B, 0x6A, 0x69, 0x68, 0x67,
            0x66, 0x65, 0x64, 0x63, 0x62,
            0x61,
        ];

        #[rustfmt::skip]
        let cases = &[
            (core::Rank::R128, core::Param::with_r128(0x6162636465666768696A6B6C6D6E6F70)),
            (core::Rank::R64, core::Param::with_r64(0x7172737475767778)),
            (core::Rank::R32, core::Param::with_r32(0x797A7B7C)),
            (core::Rank::R16, core::Param::with_r16(0x7D7E)),
            (core::Rank::R8, core::Param::with_r8(0x7F)),
        ];

        // Make 'stack'
        let mut stack = Stack::new();
        stack.inner.append(&mut init);

        // Perform 'cases'
        for (index, case) in cases.iter().enumerate() {
            // Perform test
            let obtained = stack.pop(case.0).unwrap();

            // Verify results
            let expected = case.1;
            assert_eq!(expected, obtained, "case: {}", index + 1);
        }
    }

    #[test]
    fn stack_pop_rank() {
        // Parameters
        let mut init = vec![0x7E, 0x7D, 0x7C, 0x7B, 0x7A, 0x79];

        let cases = &[
            (core::Rank::R128, None),
            (core::Rank::R64, None),
            (core::Rank::R32, Some(core::Param::with_r32(0x797A7B7C))),
            (core::Rank::R16, Some(core::Param::with_r16(0x7D7E))),
            (core::Rank::R8, None),
        ];

        // Make 'stack'
        let mut stack = Stack::new();
        stack.inner.append(&mut init);

        // Perform 'cases'
        for (index, case) in cases.iter().enumerate() {
            // Perform test
            let obtained = stack.pop(case.0);

            // Verify results
            let expected = case.1;
            assert_eq!(expected, obtained, "case: {}", index + 1);
        }
    }

    #[test]
    fn stack_save() {
        // Parameters
        let mut init = vec![0; 6];

        let expected = vec![0x44, 0x7A, 0x55, 0x7C, 0x66, 0x7E];

        let params = &[
            (core::Index::new(3), core::Param::with_r16(0x7978)),
            (core::Index::new(2), core::Param::with_r16(0x7A7B)),
            (core::Index::new(1), core::Param::with_r16(0x7C7D)),
            (core::Index::new(0), core::Param::with_r16(0x7E7F)),
            (core::Index::new(7), core::Param::with_r8(0x33)),
            (core::Index::new(5), core::Param::with_r8(0x44)),
            (core::Index::new(3), core::Param::with_r8(0x55)),
            (core::Index::new(1), core::Param::with_r8(0x66)),
        ];

        // Make 'stack'
        let mut stack = Stack::new();
        stack.inner.append(&mut init);

        // Perform test
        for param in params {
            stack.save(param.0, param.1);
        }

        // Verify results
        let obtained = &stack.inner;
        assert_eq!(&expected, obtained)
    }

    #[test]
    fn stack_load() {
        // Parameters
        let mut init = vec![
            0x7F, 0x7E, // Index 2 in Rank 16, Index 0 in Vec
            0x7D, 0x7C, // Index 1 in Rank 16, Index 1 in Vec
            0x7B, 0x7A, // Index 0 in Rank 16, Index 2 in Vec
        ];

        let cases = &[
            (core::Index::new(5), None),
            (core::Index::new(4), None),
            (core::Index::new(3), None),
            (core::Index::new(2), Some(core::Param::with_r16(0x7E7F))),
            (core::Index::new(1), Some(core::Param::with_r16(0x7C7D))),
            (core::Index::new(0), Some(core::Param::with_r16(0x7A7B))),
        ];

        // Make 'stack'
        let mut stack = Stack::new();
        stack.inner.append(&mut init);

        // Perform 'cases'
        for (index, case) in cases.iter().enumerate() {
            // Perform test
            let obtained = stack.load(case.0, core::Rank::R16);

            // Verify results
            let expected = case.1;
            assert_eq!(expected, obtained, "case: {}", index + 1);
        }
    }
}
