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
}

fn font_family() -> egui::FontFamily {
    egui::FontFamily::Name("Kaisei Opti".into())
}

pub fn rich_txt(text: impl Into<String>) -> RichText {
    RichText::new(text).font(FontId::new(16.0, font_family()))
}

pub fn rich_small_text(text: impl Into<String>) -> RichText {
    RichText::new(text).font(FontId::new(13.0, font_family()))
}

pub fn rich_title(text: impl Into<String>) -> RichText {
    RichText::new(text).font(FontId::new(22.0, font_family()))
}
