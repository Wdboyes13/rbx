mod main;
mod menu_bar;
mod solve_viewer;
mod timer;

use eframe::egui;
use crate::app::View;
use crate::app::RbxApp;

impl eframe::App for RbxApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        match self.view {
            View::Main => {
                self.main_ui(ui, frame);
            },
            View::SolveViewer => {
                self.solve_viewer_ui(ui, frame);
            },
            View::Timer => {
                self.timer_ui(ui, frame);
            }
        }
    }
}