use eframe::egui::{self, Ui};
use game_core::{
    container_args::{CharData, ContainerArgs, EnemyData},
    game_state::GameResult,
};

use crate::{
    core_wrapper::CoreWrapper,
    game_screen::draw_game_screen,
    game_state_actor::{ScreenActor, get_screen_actor},
    image_loader::ImageLoader,
    text::{set_font, txt},
    ui_state::UiStateContainer,
};

mod core_wrapper;
mod game_screen;
mod game_state_actor;
mod image_loader;
mod text;
mod ui_state;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "egui example: global font style",
        options,
        Box::new(|cc| {
            // Box::<SimpleApp>::default();
            Ok(Box::new(SimpleApp::new(cc)))
        }),
    )
}

fn get_args() -> ContainerArgs {
    ContainerArgs {
        chars: vec![
            CharData {
                level: 1.0,
                own_skill_ids: vec![0],
                static_char_id: 0,
            },
            CharData {
                level: 1.0,
                own_skill_ids: vec![0],
                static_char_id: 1,
            },
            CharData {
                level: 1.0,
                own_skill_ids: vec![0],
                static_char_id: 2,
            },
            CharData {
                level: 1.0,
                own_skill_ids: vec![0],
                static_char_id: 3,
            },
        ],
        enemy: EnemyData {
            level: 1.0,
            static_enemy_id: 0,
        },
    }
}

struct SimpleApp {
    // core: Option<CoreWrapper>,
    // screen_actor: ScreenActor,
    page: Page,
    next_page_tmp: Option<Page>,
    image_loader: ImageLoader,
    ui_state: UiStateContainer,
}

impl SimpleApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        set_font(cc);
        // let screen = get_screen_actor();
        // let (screen_actor, sender) = get_screen_actor();
        // let core = CoreWrapper::new(&get_args(), sender);
        // core.start_game();

        Self {
            next_page_tmp: None,
            page: Page::Home(GameResult::None),
            image_loader: ImageLoader::new(),
            ui_state: UiStateContainer::new(),
        }
    }
}

enum Page {
    Game(Box<CoreWrapper>, ScreenActor),
    Home(GameResult),
}

#[derive(Debug, Clone)]
struct CustomContext<'a> {
    pub ctx: &'a egui::Context,
    pub core: &'a CoreWrapper,
    pub log: &'a [String],
    pub img_loader: &'a ImageLoader,
    pub ui_state: &'a UiStateContainer,
}

fn set_space(_ui: &mut Ui) {
    // let style = &mut ui.style_mut().spacing;

    // // style.button_padding = vec2(0.0, 0.0);
    // style.indent = 0.0;
    // style.item_spacing = vec2(0.0, 0.0);
    // style.window_margin = egui::Margin {
    //     left: 0.0,
    //     right: 0.0,
    //     top: 0.0,
    //     bottom: 0.0,
    // };
}

impl eframe::App for SimpleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(n_page) = self.next_page_tmp.take() {
            self.page = n_page;
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            //// aaa
            set_space(ui);
            match &mut self.page {
                Page::Game(core, screen_actor) => {
                    core.update().unwrap();
                    screen_actor.update();
                    if screen_actor.get_result().ended() {
                        self.next_page_tmp = Some(Page::Home(screen_actor.get_result()));
                    }
                    let custom_ctx = CustomContext {
                        core,
                        ctx,
                        log: screen_actor.get_log().as_slices().0,
                        img_loader: &self.image_loader,
                        ui_state: &self.ui_state,
                    };
                    draw_game_screen(ui, &custom_ctx);
                }
                Page::Home(result) => {
                    home_page(ui, *result, self);
                }
            };

            // aaah
        });
    }
}

fn home_page(ui: &mut Ui, result: GameResult, app: &mut SimpleApp) {
    let res_text = match result {
        GameResult::Lose => "LOSE",
        GameResult::None => "",
        GameResult::Win => "WIN",
    };

    ui.label(txt(res_text));

    if ui.button(txt("開始")).clicked() {
        let (screen_actor, sender) = get_screen_actor();
        let core = CoreWrapper::new(&get_args(), sender);
        core.start_game();

        let page = Page::Game(Box::new(core), screen_actor);
        app.next_page_tmp = Some(page);
    }
}
