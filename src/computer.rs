mod display;
mod memory;
mod types;

use types::*;
pub(crate) use {display::Display, memory::Memory};

const FONTSET_SIZE: usize = 80;
const fn FONTSET() -> [Byte; FONTSET_SIZE] {
    [
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
    ]
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Computer {
    pub(crate) memory: Memory,
    pub(crate) display: Display,
    stack: [u16; 12],
    v: [u8; 0xf],
    pc: u16,
    i: u16,
    dt: u8,
    st: u8,
}

impl Computer {
    const PROGRAM_START: u16 = 0x200;

    fn boot(&mut self) {
        self.pc = Self::PROGRAM_START;
        self.i = 0;
    }
    fn load_fontset(&mut self) {
        for i in 0..80 {
            self.memory[i] = 0;
        }
    }
}

impl Computer {
    pub(crate) fn tic(&mut self) {
        // fetch
        let opcode = {
            let upper = self.memory[self.pc] as u16;
            let lower = self.memory[self.pc + 1] as u16;
            (upper << 8) | lower
        };

        let nnn = opcode & 0xfff;
        let x = (opcode & 0xf00) >> 8;
        let y = (opcode & 0xf0) >> 4;
        let n = opcode & 0xf;
        let nn = opcode & 0xff;

        // decode and execute
        match opcode & 0xf000 {
            0x0000 => match opcode {
                // 00E0: clears screen
                0x00e0 => todo!(),
                // 00EE: returns from subroutine
                0x00ee => todo!(),
                // 0NNN: calls machine code at NNN
                _ => todo!(),
            },
            // 1NNN: jumps to NNNN
            0x1000 => todo!(),
            // 2NNN: calls subroutine at NNNN
            0x2000 => {
                // self.stack.push(self.pc);
                self.pc = opcode & 0xfff;
            }
            // 3XNN: skips the next instruction if V[X] == NN
            0x3000 => todo!(),
            // 4XNN: skips the next instruction if V[X] != NN
            0x4000 => todo!(),
            // 5XY0: skips the next instruction if V[X] == V[Y]
            0x5000 => todo!(),
            // 6XNN: sets V[X] to NN
            0x6000 => todo!(),
            // 7XNN: adds NN to V[X]
            0x7000 => todo!(),
            0x8000 => match opcode & 0xf00f {
                // 8XY0: sets V[X] to V[Y]
                0x8000 => todo!(),
                // 8XY1: sets V[X] to V[X] | V[Y]
                0x8001 => todo!(),
                // 8XY2: sets V[X] to V[X] & V[Y]
                0x8002 => todo!(),
                // 8XY3: sets V[X] to V[X] ^ V[Y]
                0x8003 => todo!(),
                // 8XY4: sets V[X] to V[X] + V[Y]
                0x8004 => {
                    // if overflow then 1 else 0
                    self.v[0xf] = if x > 0xff - x { 1 } else { 0 };
                    self.v[x as usize] += self.v[y as usize];
                    self.pc += 2;
                }
                // 8XY5: sets V[X] to V[X] - V[Y]
                0x8005 => todo!(),
                // 8XY6: sets V[X] to V[X] >>= 1
                0x8006 => todo!(),
                // 8XY7: sets V[X] to V[Y] - V[X]
                0x8007 => todo!(),
                // 8XYE: sets V[X] to V[X] <<= 1
                0x800e => todo!(),
                _ => unreachable!("unknown opcode: {:03x}", opcode),
            },
            // 9XY0: skips the next instruction if V[X] != V[Y]
            0x9000 => todo!(),
            // ANNN: sets I to NNN
            0xa000 => {
                self.i = opcode & 0xfff;
                self.pc += 2;
            }
            // BNNN: jumps to NNN + V[0]
            0xb000 => todo!(),
            // CXNN: sets V[X] to rand() & NN
            0xc000 => todo!(),
            // DXYN: draw(V[X], V[Y], N)
            0xd000 => {
                /*
                let vf = &mut self.v[0xf];
                let vx = self.v[x as usize].clone();
                let vy = self.v[y as usize].clone();
                self.display
                    .update(vx, vy, n, vf, &FONTSET()[vy as usize..]);
                    */
            }
            0xe000 => match opcode & 0xf0ff {
                // EX90: skips the next instruction if V[X] == key()
                0xe09e => todo!(),
                // EXA1: skips the next instruction if V[X] != key()
                0xe0a1 => todo!(),
                _ => unreachable!("unknown opcode: {:03x}", opcode),
            },
            0xf000 => match opcode & 0xf0ff {
                // FX07: sets V[X] to DelayTimer
                0xf007 => todo!(),
                // FX0A: sets V[X] to key()
                0xf00a => todo!(),
                // FX15: sets DelayTimer to V[X]
                0xf015 => todo!(),
                // FX18: sets SoundTimer to V[X]
                0xf018 => todo!(),
                // FX1E: sets I to V[X] + I
                0xf01e => todo!(),
                // FX29: sets I to sprite(V[X])
                0xf029 => todo!(),
                // FX33: BCD(V[X])
                0xf033 => {
                    self.memory[self.i + 0] = self.v[x as usize] / 100;
                    self.memory[self.i + 1] = (self.v[x as usize] / 10) % 10;
                    self.memory[self.i + 2] = self.v[x as usize] % 10;
                }
                // FX55: reg_dump(V[X], I)
                0xf055 => todo!(),
                // FX66: reg_load(V[X], I)
                0xf066 => todo!(),
                _ => unreachable!("unknown opcode: {:03x}", opcode),
            },
            _ => unreachable!("unknown opcode: {:03x}", opcode),
        }

        // timer
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            if self.st == 1 {
                println!("beep");
            }
            self.st -= 1;
        }
    }
}

impl Computer {}
