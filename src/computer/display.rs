use egui::Vec2;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Display {
    pixels: [bool; DISPLAY_HEIGHT * DISPLAY_WIDTH],
    t: usize,
}

impl Display {
    pub(crate) const fn new() -> Self {
        Self {
            pixels: [false; DISPLAY_HEIGHT * DISPLAY_WIDTH],
            t: 0,
        }
    }
    pub(crate) fn update(&mut self) -> &mut Self {
        self.pixels[self.t % (DISPLAY_HEIGHT * DISPLAY_WIDTH)] =
            !self.pixels[self.t % (DISPLAY_HEIGHT * DISPLAY_WIDTH)];
        self.t += 1;
        self
    }
}

impl egui::Widget for Display {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::focusable_noninteractive());
        let pixel = response.rect.size() / Vec2::new(DISPLAY_WIDTH as f32, DISPLAY_HEIGHT as f32);
        for (posvec, color) in self.pixels.into_iter().enumerate().map(|(i, state)| {
            (
                Vec2::new((i % DISPLAY_WIDTH) as f32, (i / DISPLAY_WIDTH) as f32),
                if state {
                    egui::Color32::WHITE
                } else {
                    egui::Color32::BLACK
                },
            )
        }) {
            painter.add(egui::epaint::Shape::rect_filled(
                egui::Rect::from_min_size(response.rect.left_top() + posvec * pixel, pixel),
                egui::Rounding::none(),
                color,
            ));
        }
        response
    }
}

