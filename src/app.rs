use crate::chip8::{MemoryDebuger, Chip8};

pub struct MainApp {
    chip8: Chip8,
    debug: [(Box<dyn DebugWindow>, bool); 1],
}

impl MainApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.request_repaint();

        Self {
            chip8: Chip8::default(),
            debug: [(Box::new(MemoryDebuger::default()), false)],
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.chip8.update();

        egui::TopBottomPanel::top("debug").show(ctx, |ui| {
            for (window, is_open) in self.debug.iter_mut() {
                ui.toggle_value(is_open, window.name());
                window.show(ui.ctx(), is_open);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| ui.add(*self.chip8.display.lock().unwrap()));
    }
}

pub(crate) trait DebugWindow {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
