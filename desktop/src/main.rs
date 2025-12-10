use game_core6::{
    game_core_actor::{GameCoreActor, GameCoreActorCommand},
    state::GameState,
};

use iced::widget::Container;

use crate::{game_assets::new_game_core, ui_state::UIState, view::game_view};
mod common;
mod game_assets;
mod ui_state;
mod view;

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .font(include_bytes!("../assets/fonts/KaiseiOpti-Regular.ttf"))
        .default_font(iced::Font {
            family: iced::font::Family::Name("Kaisei Opti"),
            weight: iced::font::Weight::Normal,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::Normal,
        })
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    GameCoreMessage(GameCoreActorCommand),
    UiStateUpdateMessage(ui_state::UiStateUpdateMessage),
}

#[derive(Debug, Clone, Copy)]
struct Ctx<'a> {
    game_state: &'a GameState,
    ui_state: &'a UIState,
}

#[derive(Debug)]
struct App {
    game_core: GameCoreActor,
    ui_state: UIState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game_core: new_game_core(),
            ui_state: UIState::new(),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::GameCoreMessage(cmd) => {
                self.game_core.send_cmd(cmd);
                while let Some(output) = self.game_core.forward() {
                    println!("{:?}", output);
                }
            }
            Message::UiStateUpdateMessage(msg) => {
                self.ui_state.update(msg);
            }
        }
    }

    fn view(&'_ self) -> Container<'_, Message> {
        let ctx = Ctx {
            game_state: self.game_core.state(),
            ui_state: &self.ui_state,
        };

        game_view(ctx)
    }
}
