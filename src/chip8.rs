use std::sync::Mutex;

use self::{
    display::{Display, DISPLAY},
    memory::{Memory, MEMORY},
};

pub(crate) use memory::MemoryDebuger;

mod display;
mod memory;
mod sprite;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Chip8 {
    pub(crate) display: &'static Mutex<Display>,
    pub(crate) memory: &'static Mutex<Memory>,
    i: usize,
    y: usize,
    x: usize,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            display: &DISPLAY,
            memory: &MEMORY,
            i: 0,
            y: 0,
            x: 0,
        }
    }
}

impl Chip8 {
    pub(crate) fn update(&mut self) {}
}
