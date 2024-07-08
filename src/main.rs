use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    let mut text = String::from("mgrru");
    eframe::run_simple_native("mru_desk", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&text);
            if ui.button("submit").clicked() {
                text = "ru".into();
            }
            if ui.button("cancel").clicked() {
                text = "mgrru".into();
            }
        });
    })
}
