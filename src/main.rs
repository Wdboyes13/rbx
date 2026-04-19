mod scramble;
mod solve_store;
mod app;
use eframe::egui;

fn display_error(title: &str, msg: &str) {
    native_dialog::MessageDialogBuilder::default()
        .set_level(native_dialog::MessageLevel::Error)
        .set_title(title)
        .set_text(msg)
        .alert()
        .show()
        .expect("Failed to show error");
}

fn main() {
    let icon = egui::IconData {
        rgba: image::load_from_memory(include_bytes!("../icon_1024.png")).unwrap().to_rgba8().to_vec(),
        width: 1024,
        height: 1024
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([512.0, 360.0]).with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "RBX",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<app::RbxApp>::default())
        }),
    ).expect("Failed to run UI");
}