use eframe::egui::{self, RichText};
use crate::display_error;
use crate::app::View;
use crate::app::RbxApp;
use egui_extras::{TableBuilder, Column};

impl RbxApp {
    fn menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::MenuBar::new()
            .ui(ui, |ui| {
                ui.add_space(2.0);
                match self.view {
                    View::Main => {
                        if ui.button("Solve Viewer").clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::InnerSize(
                                egui::vec2(850.0, 425.0)
                            ));
                            self.view = View::SolveViewer;
                        }
                    },
                    View::SolveViewer => {
                        if ui.button("Back").clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::InnerSize(
                                egui::vec2(512.0, 360.0)
                            ));
                            self.view = View::Main;
                        }
                    }
                }
            });
    }

    fn main_ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
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
                ui.toggle_value(&mut self.scramble_vis, "Scramble visible");
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

    fn monospace(&self, txt: impl Into<String>) -> egui::RichText {
        RichText::new(txt).monospace()
    }

    fn solve_viewer_ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.menu_bar(ui);
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let row_idx_col = Column::auto().resizable(true);
            let rec_at_col = Column::initial(200.0).resizable(true);
            let sv_time_col = Column::initial(100.0).resizable(true);
            TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .column(row_idx_col)
                .column(rec_at_col)
                .column(sv_time_col)
                .column(Column::remainder())
                .header(20.0, |mut header| {
                    header.col(|ui| { ui.label("#"); });
                    header.col(|ui| { ui.label("Recorded at"); });
                    header.col(|ui| { ui.label("Solve time (sec)"); });
                    header.col(|ui| { ui.label("Scramble used"); });
                })
                .body(|mut body| {
                    let row_height = 18.0;
                    for row_idx in 0..self.solve_store.solves.len() {
                        body.row(row_height, |mut row| {
                            let sv = self.solve_store.solves[row_idx].clone();
                            row.col(|ui| { ui.label(self.monospace(row_idx.to_string())); });
                            row.col(|ui| { ui.label(self.monospace(self.unixt_to_string(sv.recorded_at))); });
                            row.col(|ui| { ui.label(self.monospace(format!("{:.03}", sv.solve_time))); });
                            row.col(|ui| { ui.label(self.monospace(sv.scramble.to_string())); });
                        });
                    }
                });
        });
    }
}

impl eframe::App for RbxApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        match self.view {
            View::Main => {
                self.main_ui(ui, frame);
            },
            View::SolveViewer => {
                self.solve_viewer_ui(ui, frame);
            }
        }
    }
}