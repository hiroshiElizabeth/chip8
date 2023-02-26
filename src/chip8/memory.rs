use std::{
    ops::{Index, IndexMut},
    sync::Mutex,
};

use crate::app::DebugWindow;

const SIZE: usize = 4096;

const SPRITE_ADDR: usize = 0x050;
const PROGRAM_ADDR: usize = 0x200;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Memory([u8; SIZE]);

impl Memory {
    const fn new() -> Self {
        use super::sprite;

        let mut data = [0; SIZE];
        data = sprite::load(data, SPRITE_ADDR);

        Self(data)
    }
}

impl Index<usize> for Memory {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.get(index).unwrap()
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.get_mut(index).unwrap()
    }
}

pub(crate) static MEMORY: Mutex<Memory> = Mutex::new(Memory::new());

#[derive(Clone, Copy)]
pub(crate) struct MemoryDebuger {
    memory: &'static Mutex<Memory>,
    col: usize,
}

impl Default for MemoryDebuger {
    fn default() -> Self {
        Self {
            memory: &MEMORY,
            col: 4,
        }
    }
}

impl DebugWindow for MemoryDebuger {
    fn name(&self) -> &'static str {
        "Memory"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name()).open(open).show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([true, false])
                .show_rows(
                    ui,
                    ui.text_style_height(&egui::TextStyle::Monospace),
                    SIZE / self.col,
                    |ui, row_range| {
                        for row in row_range.map(|r| r * self.col) {
                            ui.label(
                                egui::RichText::new(format!(
                                    "0x{row:03X}: {}",
                                    self.memory
                                        .lock()
                                        .unwrap()
                                        .0
                                        .iter()
                                        .skip(row)
                                        .take(self.col)
                                        .map(|byte| format!("0x{byte:03X}"))
                                        .collect::<Vec<_>>()
                                        .join(" "),
                                ))
                                .color(egui::Color32::WHITE)
                                .background_color(egui::Color32::BLACK)
                                .size(14.0)
                                .monospace(),
                            );
                        }
                    },
                );
        });
    }
}
