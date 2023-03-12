use egui::Key;

use crate::app::DebugWindow;

const SIZE: usize = 16;

const KEYMAP: [(Key, Key); SIZE] = [
    (Key::Num1, Key::Num1),
    (Key::Num2, Key::Num2),
    (Key::Num3, Key::Num3),
    (Key::C, Key::Num4),
    (Key::Num4, Key::Q),
    (Key::Num5, Key::W),
    (Key::Num6, Key::E),
    (Key::D, Key::R),
    (Key::Num7, Key::A),
    (Key::Num8, Key::S),
    (Key::Num9, Key::D),
    (Key::E, Key::F),
    (Key::A, Key::Z),
    (Key::Num0, Key::X),
    (Key::B, Key::C),
    (Key::F, Key::V),
];

pub(crate) fn current_key(ui: &mut egui::Ui) -> Option<u8> {
    // ui.input(|i| i.key_pressed())
}

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct KeyPad;

impl DebugWindow for KeyPad {
    fn name(&self) -> &'static str {
        "KeyPad"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .default_size(egui::vec2(150.0, 150.0))
            .show(ctx, |ui| ui.add(*self));
    }
}

impl egui::Widget for KeyPad {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Grid::new("keypad")
            .num_columns(4)
            .show(ui, |ui| {
                for y in 0..4 {
                    for x in 0..4 {
                        ui.add(egui::Button::new(
                            egui::RichText::new(KEYMAP[y * 4 + x].0.symbol_or_name())
                                .monospace()
                                .color(egui::Color32::BLACK)
                                .background_color(
                                    if ui.input(|i| i.key_pressed(KEYMAP[y * 4 + x].1)) {
                                        egui::Color32::RED
                                    } else {
                                        egui::Color32::WHITE
                                    },
                                )
                                .size(20.0),
                        ));
                    }
                    ui.end_row();
                }
            })
            .response
    }
}
