use crate::computer::Computer;

#[derive(Default)]
pub struct MainApp {
    computer: Computer,
    debug: bool,
}

impl MainApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut this = Self::default();
        this.debug = true;
        this
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            mut computer,
            mut debug,
        } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        computer.tic();

        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.toggle_value(&mut debug, "memory");
            ui.separator();
            ui.add(self.computer.display);
        });

        egui::Window::new("memory debug")
            .open(&mut self.debug)
            .movable(true)
            .vscroll(true)
            .default_height(200.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.add(computer.memory);
            });
    }
}
