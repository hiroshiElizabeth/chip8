pub(crate) type Byte = u8;

pub(crate) mod address {
    use std::ops::{Add, AddAssign};

    const MASK: usize = 0xfff;

    #[derive(Default, Debug, Clone, Copy)]
    pub(crate) struct Address(usize);

    impl Address {
        pub(crate) const fn new(addr: usize) -> Self {
            Self(addr)
        }
        pub(crate) const fn get(self) -> usize {
            self.0 & MASK
        }
    }

    impl Add<usize> for Address {
        type Output = Self;
        fn add(self, rhs: usize) -> Self::Output {
            Self::new(self.0 + rhs)
        }
    }

    impl AddAssign<usize> for Address {
        fn add_assign(&mut self, rhs: usize) {
            self.0 += rhs;
        }
    }

    impl From<usize> for Address {
        fn from(value: usize) -> Self {
            Self::new(value)
        }
    }
}
