use eframe::egui;
use game_core3::{
    args::{CharData, ContainerArgs, EnemyData},
    buttle_enemy::static_datas::StaticEnemyId,
    skill::StaticSkillId,
    static_char::StaticCharId,
};

use crate::{
    game_page::{GamePageState, draw::draw_game_screen},
    state::Page,
    text::set_font,
};

fn arg() -> ContainerArgs {
    ContainerArgs {
        chars: vec![
            CharData {
                level: 1,
                static_char_id: StaticCharId::Elena,
                own_skill_ids: vec![StaticSkillId::Fireball],
            },
            CharData {
                level: 1,
                static_char_id: StaticCharId::Asya,
                own_skill_ids: vec![StaticSkillId::Fireball],
            },
        ],
        enemy: vec![vec![
            EnemyData {
                id: StaticEnemyId::Goblin,
                level: 1,
            },
            EnemyData {
                id: StaticEnemyId::Goblin,
                level: 1,
            },
        ]],
    }
}

pub struct App {
    page: Page,
}
impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        set_font(cc);
        let game = GamePageState::new(&arg()).unwrap();
        game.sender().game_start().unwrap();
        Self {
            page: Page::GamePage(game),
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match &mut self.page {
            Page::GamePage(state) => {
                draw_game_screen(ui, ctx.screen_rect(), state);
            }
            Page::HomePage(_) => {}
        });
    }
}
