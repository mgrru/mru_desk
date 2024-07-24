use eframe::egui;
use egui::{Color32, RichText};
use fonts::load_font;
mod fonts;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "rupic",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp {
    // text: String,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        load_font(&cc.egui_ctx);
        Self {
            // text: "Edit this text field if you want".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui
                .button(
                    RichText::new("上传")
                        .color(Color32::RED)
                        .size(24.0),
                )
                .clicked()
            {};
            if ui.button("存储").clicked() {};
            if ui.button("设置").clicked() {};
            if ui.button("关于").clicked() {};
        });
    }
}
