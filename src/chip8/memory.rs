use std::sync::Mutex;

use crate::app::DebugWindow;

type Byte = u8;

const SIZE: usize = 4096;

const SPRITE_ADDR: usize = 0x050;
const PROGRAM_ADDR: usize = 0x200;

pub(crate) static MEMORY: Mutex<core::Memory> = Mutex::new(core::Memory::new().init());

pub(crate) mod core {
    use std::ops::{Index, IndexMut, Range};

    use super::{Byte, SIZE, SPRITE_ADDR};

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct Memory([Byte; SIZE]);

    impl Memory {
        pub(super) const fn new() -> Self {
            Self([0; SIZE])
        }
        pub(super) const fn init(self) -> Self {
            use super::super::sprite;
            Self(sprite::load(self.0, SPRITE_ADDR))
        }
    }

    impl Index<usize> for Memory {
        type Output = Byte;
        fn index(&self, index: usize) -> &Self::Output {
            self.0.get(index).unwrap()
        }
    }

    impl IndexMut<usize> for Memory {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            self.0.get_mut(index).unwrap()
        }
    }

    impl Index<Range<usize>> for Memory {
        type Output = [Byte];
        fn index(&self, index: Range<usize>) -> &Self::Output {
            self.0.get(index).unwrap()
        }
    }

    impl IndexMut<Range<usize>> for Memory {
        fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
            self.0.get_mut(index).unwrap()
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct MemoryDebuger {
    memory: &'static Mutex<core::Memory>,
    style: DataStyle,
    trace_addr: usize,
    trace_align: Option<egui::Align>,
}

impl Default for MemoryDebuger {
    fn default() -> Self {
        Self {
            memory: &MEMORY,
            style: DataStyle::default(),
            trace_addr: 0x000,
            trace_align: Some(egui::Align::TOP),
        }
    }
}

impl DebugWindow for MemoryDebuger {
    fn name(&self) -> &'static str {
        "Memory"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .auto_sized()
            .show(ctx, |ui| {
                let mut tracing = false;

                ui.horizontal(|ui| {
                    ui.label("format: ");
                    ui.radio_value(&mut self.style, DataStyle::Bin, "bin")
                        .clicked();
                    ui.radio_value(&mut self.style, DataStyle::Hex, "hex")
                        .clicked();
                });

                ui.horizontal(|ui| {
                    ui.label("addr: ");
                    tracing |= ui
                        .add(
                            egui::widgets::Slider::new(&mut self.trace_addr, 0..=SIZE - 1)
                                .integer()
                                .hexadecimal(3, false, true)
                                .prefix("0x"),
                        )
                        .dragged();
                });

                ui.horizontal(|ui| {
                    ui.label("special addr: ");
                    if ui.button("zero").clicked() {
                        self.trace_addr = 0;
                        tracing = true;
                    }
                    if ui.button("sprite").clicked() {
                        self.trace_addr = SPRITE_ADDR;
                        tracing = true;
                    }
                    if ui.button("program").clicked() {
                        self.trace_addr = PROGRAM_ADDR;
                        tracing = true;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Item align:");
                    tracing |= ui
                        .radio_value(&mut self.trace_align, Some(egui::Align::Min), "Top")
                        .clicked();
                    tracing |= ui
                        .radio_value(&mut self.trace_align, Some(egui::Align::Center), "Center")
                        .clicked();
                    tracing |= ui
                        .radio_value(&mut self.trace_align, Some(egui::Align::Max), "Bottom")
                        .clicked();
                    tracing |= ui
                        .radio_value(&mut self.trace_align, None, "None")
                        .clicked();
                });

                ui.separator();

                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        for addr in (0..SIZE).filter(|addr| addr % self.style.col() == 0) {
                            let text = egui::RichText::new(format!(
                                "0x{addr:03X}: {}",
                                self.memory.lock().unwrap()[addr..addr + self.style.col()]
                                    .iter()
                                    .map(|&byte| self.style.to_string(byte))
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            ))
                            .size(16.0)
                            .monospace();

                            if tracing
                                && addr <= self.trace_addr
                                && self.trace_addr < addr + self.style.col()
                            {
                                let response = ui.colored_label(egui::Color32::YELLOW, text);
                                response.scroll_to_me(self.trace_align);
                            } else {
                                ui.label(text);
                            }
                        }
                    });
            });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DataStyle {
    Hex,
    Bin,
}

impl Default for DataStyle {
    fn default() -> Self {
        Self::Hex
    }
}

impl DataStyle {
    const fn col(self) -> usize {
        match self {
            Self::Hex => 4,
            Self::Bin => 2,
        }
    }
    fn to_string(self, byte: Byte) -> String {
        match self {
            Self::Hex => format!("0x{byte:03X}"),
            Self::Bin => format!("0b{byte:08b}"),
        }
    }
}
