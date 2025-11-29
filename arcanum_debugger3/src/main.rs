use std::borrow::Cow;

mod state;
mod app;
mod game_page;
mod text;



fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Arcanum Debugger",
        options,
        Box::new(|cc| {
            Ok(Box::new(app::App::new(cc)))
        }),
    )
}
