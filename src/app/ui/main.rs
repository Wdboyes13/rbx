use eframe::egui::{self, RichText};
use crate::display_error;
use crate::app::RbxApp;

impl RbxApp {
    pub fn main_ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.menu_bar(ui);
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label(RichText::new("Current scramble").size(16.0)); 
            if self.scramble_vis {
                ui.label(self.current_scramble.to_string());
            } else {
                ui.label("Scramble Hidden");
            }

            ui.add_space(5.0);
            ui.label(RichText::new("Scramble Options").size(16.0));
            ui.indent(0, |ui| {
                let toggle_txt = if self.scramble_vis { "Hide scramble" } else { "Show scramble" };
                ui.toggle_value(&mut self.scramble_vis, toggle_txt);
                ui.horizontal(|ui| {
                    ui.label("Scramble length:");
                    ui.add(egui::DragValue::new(&mut self.sparm_len).speed(1).range(1..=100));
                });
                ui.horizontal(|ui| {
                    ui.label("Cube size:");
                    ui.add(egui::DragValue::new(&mut self.sparm_cx).speed(1).range(1..=100));
                });
                if ui.button("Rescramble").clicked() {
                    self.rescramble();
                }
            });
            
            ui.add_space(5.0);
            ui.label(RichText::new("Solve store").size(16.0));
            ui.indent(0, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Solve time:");
                    ui.text_edit_singleline(&mut self.stime_buf);
                });
                if ui.button("Store solve").clicked() {
                    if let Ok(stime) = self.stime_buf.parse::<f64>() {
                        self.push_current(stime);
                        self.stime_buf.clear();
                    } else {
                        display_error("Parse error", "Input time could not be converted to a number");
                    }
                }
            });

            ui.add_space(5.0);
            ui.label(RichText::new("Stats").size(16.0));
            ui.indent(0, |ui| {
                match self.ao5 {
                    Some(ao5) => {
                        ui.label(format!("Ao5: {:.03}", ao5));
                    },
                    None => {
                        ui.label("Ao5: Not enough solves");
                    }
                }

                match self.ao12 {
                    Some(ao12) => {
                        ui.label(format!("Ao12: {:.03}", ao12));
                    },
                    None => {
                        ui.label("Ao12: Not enough solves");
                    }
                }
                
                ui.label(format!("Best: {:.03}", self.best));
                ui.label(format!("Worst: {:.03}", self.worst));
            });
        });
    }
}