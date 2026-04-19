use eframe::egui::{self, RichText};
use crate::app::RbxApp;
use egui_extras::{TableBuilder, Column};

impl RbxApp {
    fn monospace(&self, txt: impl Into<String>) -> egui::RichText {
        RichText::new(txt).monospace()
    }

    pub fn solve_viewer_ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
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