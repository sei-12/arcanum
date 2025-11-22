use eframe::egui::{self, FontData, FontDefinitions, FontFamily, FontId, RichText};

pub fn set_font(cc: &eframe::CreationContext<'_>) {
    let mut fonts = FontDefinitions::default();
    let font = FontData::from_static(include_bytes!("../assets/fonts/KaiseiOpti.ttf"));
    fonts.font_data.insert("Kaisei Opti".to_owned(), font);

    fonts
        .families
        .entry(FontFamily::Name("Kaisei Opti".into()))
        .or_default()
        .insert(0, "Kaisei Opti".to_owned());

    cc.egui_ctx.set_fonts(fonts);
    // cc.egui_ctx.setfon
}

pub fn txt(text: &str) -> RichText {
    RichText::new(text).font(FontId::new(
        20.0,
        egui::FontFamily::Name("Kaisei Opti".into()),
    ))
}
