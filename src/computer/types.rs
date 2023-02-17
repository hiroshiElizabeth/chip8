use std::ops::{Add, AddAssign};

use super::memory::MemoryAccess;

pub type Byte = u8;

pub(super) trait Nibble {
    const MASK: u8 = 0xf;
    fn upper_nibble(self) -> u8;
    fn lower_nibble(self) -> u8;
}

/// Program Counter
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PC(u16);

impl PC {
    const fn address(self) -> usize {
        (self.0 & 0xfff) as usize
    }
}

impl MemoryAccess for PC {
    fn access(self) -> usize {
        self.address()
    }
}

impl Add<u16> for PC {
    type Output = Self;
    fn add(mut self, rhs: u16) -> Self::Output {
        self.0 += rhs;
        self
    }
}

impl AddAssign<u16> for PC {
    fn add_assign(&mut self, rhs: u16) {
        self.0 += rhs;
    }
}

/// I register
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct I(u16);

impl I {
    const fn address(self) -> usize {
        (self.0 & 0xfff) as usize
    }
}

impl MemoryAccess for I {
    fn access(self) -> usize {
        self.address()
    }
}
