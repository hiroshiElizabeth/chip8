// use super::data::Address;

const SIZE: usize = 80;

const DATA: [u8; SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub(super) mod v2 {
    use super::*;

    pub(crate) fn load(data: &mut [u8]) -> usize {
        for i in 0..SIZE {
            data[i] = DATA[i];
        }
        SIZE
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn load() {
            const INIT: u8 = 0xab;
            const SIZE: usize = 100;
            const START: usize = 10;

            let data = {
                let mut data = [INIT; SIZE];
                let _size = super::load(&mut data[START..]);
                data
            };

            let mut i = 0;
            while i < START {
                assert_eq!(data[i], INIT);
                i += 1;
            }
            while i < START + super::SIZE {
                assert_eq!(data[i], super::DATA[i - START]);
                i += 1;
            }
            while i < SIZE {
                assert_eq!(data[i], INIT);
                i += 1;
            }
        }
    }
}

pub(super) const fn load<const N: usize>(mut data: [u8; N], start: usize) -> [u8; N] {
    let mut i = 0;
    while i < SIZE {
        data[start + i] = DATA[i];
        i += 1;
    }
    return data;
}

#[test]
fn t_show() {
    load([0; 80], 0).into_iter().enumerate().for_each(|(i, x)| {
        println!("{}0x{i:03x}: 0b{x:08b}", if i % 5 == 0 { "\n" } else { "" });
    });
}
