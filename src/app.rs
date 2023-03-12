use crate::chip8::display;
use crate::chip8::Chip8;

pub struct MainApp {
    chip8: Chip8,
    debug: [(Box<dyn DebugWindow>, bool); 0],
}

impl MainApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            chip8: Chip8::default(),
            debug: [],
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.chip8.update();

        egui::TopBottomPanel::top("debug").show(ctx, |ui| {
            ui.horizontal(|ui| {
                self.debug.iter_mut().for_each(|(window, is_open)| {
                    ui.toggle_value(is_open, window.name());
                    window.show(ui.ctx(), is_open);
                });
            });
        });

        ctx.request_repaint();
        // egui::CentralPanel::default().show(ctx, |ui| ui.add(*DISPLAY.lock().unwrap()));
        egui::CentralPanel::default().show(ctx, |ui| ui.add(display::Display::default()));
    }
}

pub(crate) trait DebugWindow {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
