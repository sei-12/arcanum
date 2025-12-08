use game_core6::{
    buttle_char::ButtleChar,
    buttle_enemy::ButtleEnemy,
    game_core_actor::{GameCoreActor, GameCoreActorCommand},
    lt_common::LtCommon,
    state::GameState,
};
use iced::{
    Application, Element,
    widget::{Container, Row, column, container, row, text},
};

use crate::game_assets::{get_char_name, get_enemy_name, new_game_core};
mod game_assets;

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
    // iced::run(App::update, App::view)
}

#[derive(Debug, Clone)]
enum Message {
    GameCoreMessage(GameCoreActorCommand),
}

#[derive(Debug)]
struct App {
    game_core: GameCoreActor,
}

impl Default for App {
    fn default() -> Self {
        let setting = iced::Settings {
            ..Default::default()
        };

        Self {
            game_core: new_game_core(),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::GameCoreMessage(cmd) => {
                self.game_core.send_cmd(cmd);
                while self.game_core.forward().is_some() {}
            }
        }
    }

    fn view(&'_ self) -> Container<'_, Message> {
        game_view(self.game_core.state())
    }
}

fn game_view(state: &GameState) -> Container<'_, Message> {
    container(column![enemy_side_view(state), player_side_view(state),])
}

fn enemy_side_view(state: &GameState) -> Element<'_, Message> {
    Row::with_children(
        state
            .get_current_wave_enemys()
            .iter()
            .map(|e| enemy_item_view(e)),
    )
    .into()
}

fn enemy_item_view(enemy: &ButtleEnemy) -> Element<'_, Message> {
    let enemy_name = get_enemy_name(enemy.static_data().static_id());
    column![text(enemy_name).size(20), lt_common_view(enemy.lt())].into()
}

fn player_side_view(state: &GameState) -> Row<'_, Message> {
    Row::with_children(state.get_chars().iter().map(|c| char_item_view(c).into()))
}

fn char_item_view(char: &ButtleChar) -> Container<'_, Message> {
    let char_name = get_char_name(char.static_data().id);
    container(column![text(char_name).size(20), lt_common_view(char.lt())])
}

fn lt_common_view(lt_common: &LtCommon) -> Element<'_, Message> {
    row![
        column![
            row![text!(
                "HP: {}/{}",
                lt_common.hp().round(),
                lt_common.max_hp().round()
            )],
            row![text("魔法攻撃力: "), text(lt_common.magic_attuck().round())],
            row![
                text("物理攻撃力: "),
                text(lt_common.physics_attuck().round())
            ],
            row![
                text("被魔法ダメージ倍率: "),
                text(lt_common.recv_magic_dmg_mag().round())
            ],
            row![
                text("被物理ダメージ倍率: "),
                text(lt_common.recv_physics_dmg_mag().round())
            ],
        ],
        column![
            row![text("INT"), text(lt_common.int().round())],
            row![text("AGI"), text(lt_common.agi().round())],
            row![text("STR"), text(lt_common.str().round())],
            row![text("DEX"), text(lt_common.dex().round())],
            row![text("VIT"), text(lt_common.vit().round())],
        ]
    ]
    .into()
}
