use eframe::egui::{self, RichText};
use crate::app::RbxApp;

impl RbxApp { 
    pub fn timer_ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.menu_bar(ui);
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.input(|inp| {
                if inp.key_pressed(egui::Key::Space) {
                    if self.swatch.is_running() {
                        self.swatch.stop();
                    } else {
                        self.swatch.spans.clear();
                        self.swatch.start();
                    }
                }
                if inp.key_pressed(egui::Key::C) && !self.swatch.is_running() {
                    self.swatch.spans.clear();
                }
            });

            if self.swatch.is_running() {
                ui.ctx().request_repaint_after(std::time::Duration::from_millis(10));
            }

            ui.vertical_centered_justified(|ui| {
                ui.label(
                RichText::new(self.format_swtime())
                        .size(24.0)
                        .monospace()
                );
                    
                if self.swatch.is_running() {
                    ui.label("Press [space] to stop");
                } else {
                    ui.label("Press [space] to start");
                    ui.label("Press [c] to clear time");
                }
            });
        });
    }
}