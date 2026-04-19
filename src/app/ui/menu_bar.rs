use crate::app::RbxApp;
use crate::app::View;
use eframe::egui;

impl RbxApp {
    pub fn menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::MenuBar::new()
            .ui(ui, |ui| {
                ui.add_space(2.0);
                match self.view {
                    View::Main => {
                        if ui.button("Back").clicked() &&
                            let Some(last_view) = self.last_view {
                                self.last_view = Some(View::Main);
                                self.view = last_view;
                                self.set_vpsz(ui);
                        }

                        if ui.button("Solve Viewer").clicked() {
                            self.last_view = Some(View::Main);
                            self.view = View::SolveViewer;
                            self.set_vpsz(ui);
                        }

                        if ui.button("Timer").clicked() {
                            self.last_view = Some(View::Main);
                            self.view = View::Timer;
                            self.set_vpsz(ui);
                        }
                    },
                    View::SolveViewer => {
                        if ui.button("Back").clicked() && 
                            let Some(last_view) = self.last_view {
                                self.last_view = Some(View::SolveViewer);
                                self.view = last_view;
                                self.set_vpsz(ui);
                        }

                        if ui.button("Home").clicked() {
                            self.last_view = Some(View::SolveViewer);
                            self.view = View::Main;
                            self.set_vpsz(ui);
                        }

                        if ui.button("Timer").clicked() {
                            self.last_view = Some(View::SolveViewer);
                            self.view = View::Timer;
                            self.set_vpsz(ui);
                        }
                    },
                    View::Timer => {
                        if ui.button("Back").clicked() &&
                            let Some(last_view) = self.last_view {
                                self.last_view = Some(View::Timer);
                                self.view = last_view;
                                self.set_vpsz(ui);
                        }

                        if ui.button("Home").clicked() {
                            self.last_view = Some(View::Timer);
                            self.view = View::Main;
                            self.set_vpsz(ui);
                        }

                        if ui.button("Solve Viewer").clicked() {
                            self.last_view = Some(View::Timer);
                            self.view = View::SolveViewer;
                            self.set_vpsz(ui);
                        }
                    }
                }
            });
    }
}