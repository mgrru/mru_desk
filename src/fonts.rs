use eframe::egui::{Context, FontDefinitions};

pub(super) fn load_font(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    let font_name = "envy_code_r_nerd_mono";
    let font_name2 = "chinese_font";

    fonts.font_data.insert(
        font_name2.to_string(),
        egui::FontData::from_static(include_bytes!("font/文泉驿正黑.ttc")),
    );
    fonts.font_data.insert(
        font_name.to_string(),
        egui::FontData::from_static(include_bytes!(
            "font/envycoder/EnvyCodeRNerdFontMono-Regular.ttf"
        )),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, font_name2.to_string());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, font_name.to_string());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push(font_name2.to_string());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push(font_name.to_string());

    ctx.set_fonts(fonts);
}
