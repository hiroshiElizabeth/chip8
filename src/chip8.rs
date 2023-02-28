use std::sync::Mutex;

use self::display::{Display, DISPLAY};

pub(crate) mod display;
pub(crate) mod memory;
pub(crate) mod sprite;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Chip8 {
    pub(crate) display: &'static Mutex<Display>,
    // pub(crate) memory: &'static Mutex<Memory>,
    i: usize,
    y: usize,
    x: usize,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            display: &DISPLAY,
            // memory: &MEMORY,
            i: 0,
            y: 0,
            x: 0,
        }
    }
}

impl Chip8 {
    pub(crate) fn update(&mut self) {
        self.display.lock().unwrap().flip((self.x, self.y));
        self.x += 1;
        if self.x >= 64 {
            self.x = 0;
            self.y += 1;
            if self.y >= 32 {
                self.y = 0;
            }
        }
    }
}
