use eframe::egui;
use soft_plc::editor::{PlcEditorApp, PlcNodeTemplate};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1400.0, 900.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Soft-PLC Visual Editor",
        options,
        Box::new(|cc| Box::new(PlcEditorApp::new(cc))),
    )
}
