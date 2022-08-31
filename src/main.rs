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
    icon: char,
}
impl Default for DemoApp {
    fn default() -> Self {
        Self {
            color: Color32::RED,
            icon: '⊗'
        }
    }
}

const AVAILABLE_ICONS: &'static [char] = &['⊗', '⬛', '⏺'];

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            egui::global_dark_light_mode_buttons(ui);
            egui::ComboBox::new("icon_combo", "Icon")
                .selected_text(self.icon.to_string())
                .show_ui(ui, |ui| {
                    for icon in AVAILABLE_ICONS {
                        ui.selectable_value(&mut self.icon, *icon, icon.to_string());
                    }
                });
            ui.color_edit_button_srgba(&mut self.color);
            let _ = ui.button(RichText::new(self.icon).color(self.color));
            let _ = ui.button("⊗");
        });
    }
}