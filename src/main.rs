use eframe::egui;
use egui::{Rgba, RichText};
use fonts::load_font;
mod fonts;

fn main() -> eframe::Result {
    // 创建视口选项，设置视口的内部大小为320x240像素
    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),

    //     ..Default::default()
    // };

    let options = eframe::NativeOptions::default();
    // 运行
    eframe::run_native(
        "rupic",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::new(cc)))
        }),
    )
}

struct MyApp {
    // text: String,
    name: String,
    age: u32,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        load_font(&cc.egui_ctx);
        Self {
            // text: "Edit this text field if you want".to_owned(),
            name: "ru".to_string(),
            age: 18,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 标题
            ui.heading("this is header");

            // 水平布局
            ui.horizontal(|ui| {
                let name_label = ui.label("name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("年龄:"));

            if ui
                .button(
                    RichText::new("上传")
                        .color(Rgba::from_rgba_unmultiplied(183.0, 69.0, 128.0, 0.8))
                        .size(24.0),
                )
                .clicked()
            {
                self.age += 1;
            };
            if ui.button("存储").clicked() {};
            if ui.button("设置").clicked() {};
            if ui.button("关于").clicked() {};

            ui.image(egui::include_image!("./image/fufu"));
        });
    }
}
