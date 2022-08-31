#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui::{self, RichText, Color32}};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Demo App",
        options,
        Box::new(|_cc| Box::new(DemoApp::default())),
    );
}

struct DemoApp {
    color: Color32,
}
impl Default for DemoApp {
    fn default() -> Self {
        Self {
            color: Color32::RED,
        }
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::global_dark_light_mode_buttons(ui);
            ui.color_edit_button_srgba(&mut self.color);
            let _ = ui.button(RichText::new("⊗").color(self.color));
            let _ = ui.button("⊗");
        });
    }
}