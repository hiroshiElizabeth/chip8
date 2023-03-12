mod data;
pub(crate) mod display;
pub(crate) mod memory;
pub(crate) mod register;
pub(crate) mod sprite;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Chip8 {
    y: usize,
    x: usize,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self { y: 0, x: 0 }
    }
}

impl Chip8 {
    pub(crate) fn update(&mut self) {
        display::flip((self.x, self.y));
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
