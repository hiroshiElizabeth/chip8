use std::{collections::BTreeSet, sync::Mutex};

use egui::{Pos2, Vec2, Widget};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

static UPDATE_STACK: Mutex<BTreeSet<DisplayPos>> = Mutex::new(BTreeSet::new());

pub(crate) fn flip(pos: impl Into<DisplayPos>) -> bool {
    let mut stack = UPDATE_STACK.lock().unwrap();
    let pos = pos.into();

    if !stack.insert(pos) {
        stack.remove(&pos);
        return false;
    }

    true
}

// for egui::Widget
#[derive(Debug, Clone, Copy)]
pub(crate) struct Display {
    pixels: &'static Mutex<BTreeSet<DisplayPos>>,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            pixels: &UPDATE_STACK,
        }
    }
}

impl Widget for Display {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::focusable_noninteractive());
        let cell = response.rect.size() / Vec2::new(WIDTH as f32, HEIGHT as f32);

        painter.rect_filled(
            response.rect,
            egui::Rounding::default(),
            egui::Color32::BLACK,
        );

        for &pos in self.pixels.lock().unwrap().iter() {
            let pos: egui::Pos2 = pos.into();
            painter.rect_filled(
                egui::Rect::from_min_size(response.rect.left_top() + pos.to_vec2() * cell, cell),
                egui::Rounding::none(),
                egui::Color32::WHITE,
            );
        }

        response
    }
}

// Display position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DisplayPos {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for DisplayPos {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl From<DisplayPos> for usize {
    fn from(DisplayPos { x, y }: DisplayPos) -> Self {
        x + y * WIDTH
    }
}

impl From<DisplayPos> for Pos2 {
    fn from(DisplayPos { x, y }: DisplayPos) -> Self {
        Self::new(x as f32, y as f32)
    }
}

impl PartialOrd for DisplayPos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a: usize = self.clone().into();
        let b: usize = other.clone().into();
        a.partial_cmp(&b)
    }
}

impl Ord for DisplayPos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a: usize = self.clone().into();
        let b: usize = other.clone().into();
        a.cmp(&b)
    }
}
