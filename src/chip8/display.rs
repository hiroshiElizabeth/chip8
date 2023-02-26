use std::{
    ops::{Index, IndexMut},
    sync::Mutex,
};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub(crate) static DISPLAY: Mutex<Display> = Mutex::new(Display::new());

#[derive(Debug, Clone, Copy)]
pub(crate) struct Display([bool; HEIGHT * WIDTH]);

impl Display {
    const fn new() -> Self {
        Self([false; HEIGHT * WIDTH])
    }
    const fn to_index(x: usize, y: usize) -> usize {
        x + y * WIDTH
    }
    pub(crate) fn flip(&mut self, (x, y): (usize, usize)) -> bool {
        self[(x, y)] ^= true;
        self[(x, y)]
    }
}

impl Index<(usize, usize)> for Display {
    type Output = bool;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.0.get(Self::to_index(x, y)).unwrap()
    }
}

impl IndexMut<(usize, usize)> for Display {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.0.get_mut(Self::to_index(x, y)).unwrap()
    }
}

impl egui::Widget for Display {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::focusable_noninteractive());
        let size = response.rect.size() / egui::vec2(WIDTH as f32, HEIGHT as f32);

        painter.rect_filled(response.rect, egui::Rounding::none(), egui::Color32::BLACK);

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if !self[(x, y)] {
                    continue;
                }
                painter.rect_filled(
                    egui::Rect::from_min_size(
                        response.rect.left_top() + size * egui::vec2(x as f32, y as f32),
                        size,
                    ),
                    egui::Rounding::none(),
                    egui::Color32::WHITE,
                );
            }
        }

        response
    }
}

/*
impl egui::Widget for DisplayWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        use egui::{Sense, Vec2};

        let (response, painter) =
            ui.allocate_painter(ui.available_size(), Sense::focusable_noninteractive());

        let pixel = response.rect.size() / Vec2::new(WIDTH as f32, HEIGHT as f32);

        for (y, line) in self.0.into_iter().enumerate() {
            for x in 0..64 {
                painter.rect_filled(
                    egui::Rect::from_min_size(
                        response.rect.left_top() + egui::vec2(x as f32, y as f32) * pixel,
                        pixel,
                    ),
                    egui::Rounding::none(),
                    if line & (1 << (63 - x)) == 0 {
                        egui::Color32::BLACK
                    } else {
                        egui::Color32::WHITE
                    },
                );
            }
        }

        response
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Display {
    data: [u64; HEIGHT],
}

impl Default for Display {
    fn default() -> Self {
        Self { data: [0; HEIGHT] }
    }
}

impl Display {
    const fn new() -> Self {
        Self { data: [0; HEIGHT] }
    }
    pub(super) fn update(&mut self, x: usize, y: usize, sprite: u64) -> u16 {
        let mut output = 0;
        for i in 0..8 {
            let x0 = x + i;
            if sprite & (0x80 >> i) != 0 {
                if self.data[y] & (1 << (63 - x0)) != 0 {
                    output = 1;
                }
                self.data[y] ^= 1 << (63 - x0);
            }
        }
        output
    }
}

impl egui::Widget for Display {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        use egui::{Sense, Vec2};

        let (response, painter) =
            ui.allocate_painter(ui.available_size(), Sense::focusable_noninteractive());

        let pixel = response.rect.size() / Vec2::new(WIDTH as f32, HEIGHT as f32);

        for (y, line) in self.data.into_iter().enumerate() {
            for x in 0..64 {
                painter.rect_filled(
                    egui::Rect::from_min_size(
                        response.rect.left_top() + egui::vec2(x as f32, y as f32) * pixel,
                        pixel,
                    ),
                    egui::Rounding::none(),
                    if line & (1 << (63 - x)) == 0 {
                        egui::Color32::BLACK
                    } else {
                        egui::Color32::WHITE
                    },
                );
            }
        }

        response
    }
}
*/
