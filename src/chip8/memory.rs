use std::sync::Mutex;

use std::ops::{Index, IndexMut, Range};

use super::data::address::Address;

use super::data::Byte;

const SIZE: usize = 4096;

const SPRITE_ADDR: usize = 0x050;
const PROGRAM_ADDR: usize = 0x200;

static MEMORY: Mutex<Memory> = Mutex::new(Memory::new().init());

#[derive(Debug, Clone, Copy)]
pub(crate) struct Memory([Byte; SIZE]);

impl Memory {
    pub(super) const fn new() -> Self {
        Self([0; SIZE])
    }
    pub(super) const fn init(self) -> Self {
        use super::sprite;
        Self(sprite::load(self.0, SPRITE_ADDR))
    }
}

impl<A: Into<Address>> Index<A> for Memory {
    type Output = Byte;
    fn index(&self, addr: A) -> &Self::Output {
        self.0.get(addr.into().get()).unwrap()
    }
}

impl<A: Into<Address>> IndexMut<A> for Memory {
    fn index_mut(&mut self, addr: A) -> &mut Self::Output {
        self.0.get_mut(addr.into().get()).unwrap()
    }
}

impl<A: Into<Address>> Index<Range<A>> for Memory {
    type Output = [Byte];
    fn index(&self, Range { start, end }: Range<A>) -> &Self::Output {
        self.0
            .get(Range {
                start: start.into().get(),
                end: end.into().get(),
            })
            .unwrap()
    }
}

impl<A: Into<Address>> IndexMut<Range<A>> for Memory {
    fn index_mut(&mut self, Range { start, end }: Range<A>) -> &mut Self::Output {
        self.0
            .get_mut(Range {
                start: start.into().get(),
                end: end.into().get(),
            })
            .unwrap()
    }
}
