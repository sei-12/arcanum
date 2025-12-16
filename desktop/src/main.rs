use std::time::Duration;

use game_core9::core_actor::{GameCoreActor, GameCoreOutput, UserInput};
use game_core9::game_state::GameState;
use iced::Element;

use crate::assets::game_core;
use crate::ui_state::UIState;
use crate::view::game_view;

mod assets;
mod common;
mod ui_state;
mod view;

pub fn main() -> iced::Result {
    iced::application("Pixels", Example::update, Example::view)
        // This subscription is just to make the animation work
        .subscription(|_| iced::time::every(Duration::from_millis(10)).map(|_| Message::Step))
        .font(include_bytes!("../assets/fonts/KaiseiOpti-Regular.ttf"))
        .default_font(iced::Font {
            family: iced::font::Family::Name("Kaisei Opti"),
            weight: iced::font::Weight::Normal,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::Normal,
        })
        .run()
}

struct Example {
    game_core: GameCoreActor,
    output_buffer: Vec<GameCoreOutput>,
    ui_state: UIState,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            game_core: game_core(),
            output_buffer: Vec::new(),
            ui_state: UIState::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Step,
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::Step => {
                println!("step");
                // self.game_core
                //     .tick(UserInput::None, &mut self.output_buffer)
                //     .unwrap()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let ctx = Ctx {
            game_state: self.game_core.state(),
            ui_state: &self.ui_state,
        };

        game_view(ctx).into()
    }
}

#[derive(Debug, Clone, Copy)]
struct Ctx<'a> {
    game_state: &'a GameState,
    ui_state: &'a UIState,
}
