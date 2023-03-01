pub(crate) use stack::{Stack, STACK};

mod stack {
    use std::sync::Mutex;

    pub(crate) static STACK: Mutex<Stack> = Mutex::new(Stack::new());

    type Addr = u16;

    const SIZE: usize = 12;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct Stack {
        data: [Addr; SIZE],
        pos: usize,
    }

    impl Stack {
        const fn new() -> Self {
            Self {
                data: [0; SIZE],
                pos: 0,
            }
        }
        pub(crate) fn push(&mut self, value: Addr) {
            if self.pos < SIZE {
                self.data[self.pos] = value;
                self.pos += 1;
            }
        }
        pub(crate) fn pop(&mut self) -> Option<Addr> {
            if self.pos == 0 {
                return None;
            }
            self.pos -= 1;
            Some(self.data[self.pos])
        }
    }
}

mod pc {
    use std::sync::Mutex;

    pub(crate) static PC: Mutex<ProgramCounter> = Mutex::new(ProgramCounter::new());

    type Addr = usize;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct ProgramCounter {
        addr: Addr,
    }

    impl ProgramCounter {
        const fn new() -> Self {
            Self { addr: 0 }
        }
        pub(crate) const fn get(self) -> Addr {
            self.addr
        }
        pub(crate) fn jump(&mut self, addr: Addr) {
            self.addr = addr;
        }
    }
}

mod general {
    use std::{
        ops::{Index, IndexMut},
        sync::Mutex,
    };

    pub(crate) static V: Mutex<GeneralRegister> = Mutex::new(GeneralRegister::new());

    const SIZE: usize = 16;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct GeneralRegister([u8; SIZE]);

    impl GeneralRegister {
        const fn new() -> Self {
            Self([0; SIZE])
        }
    }

    impl Index<u8> for GeneralRegister {
        type Output = u8;
        fn index(&self, index: u8) -> &Self::Output {
            self.0.get(index as usize).unwrap()
        }
    }

    impl IndexMut<u8> for GeneralRegister {
        fn index_mut(&mut self, index: u8) -> &mut Self::Output {
            self.0.get_mut(index as usize).unwrap()
        }
    }
}

mod address {
    use std::sync::Mutex;

    pub(crate) static I: Mutex<AddressRegister> = Mutex::new(AddressRegister::new());

    type Addr = u16;

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct AddressRegister {
        addr: Addr,
    }

    impl AddressRegister {
        const fn new() -> Self {
            Self { addr: 0 }
        }
    }
}

mod timer {
    pub(crate) struct DelayTimer {
        timer: usize,
    }
    pub(crate) struct SoundTimer {
        timer: usize,
    }
}
