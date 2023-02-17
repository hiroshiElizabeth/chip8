use std::ops::{Index, IndexMut};

use super::types::*;

pub(super) trait MemoryAccess {
    fn access(self) -> usize;
}

const MEMORY_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Memory([Byte; MEMORY_SIZE]);

impl Default for Memory {
    fn default() -> Self {
        // for test
        let mut a = [Byte::default(); MEMORY_SIZE];
        a[0] = 0xa2;
        a[1] = 0xf0;
        Self(a)
    }
}

impl Memory {
    const fn masked(addr: u16) -> usize {
        (addr & 0xfff) as usize
    }
}

impl Index<u16> for Memory {
    type Output = Byte;
    fn index(&self, index: u16) -> &Self::Output {
        self.0.get(Self::masked(index)).unwrap()
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        self.0.get_mut(Self::masked(index)).unwrap()
    }
}

impl egui::Widget for Memory {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Grid::new("memory")
            .striped(true)
            .show(ui, |ui| {
                for addr in 0..MEMORY_SIZE {
                    ui.label(format!("0x{:03X}", addr));
                    ui.label(format!("0x{:02X}", self.0[addr]));
                    ui.label(format!("0b{:08b}", self.0[addr]));
                    ui.end_row();
                }
            })
            .response
    }
}
