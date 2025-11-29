// use eframe::egui::Ui;

use crate::{
    game_page::GamePageState,
    text::{rich_small_text, rich_title, rich_txt},
};

// pub fn draw_game_screen(page_state: &mut GamePageState, ui: &mut Ui){

// }
use eframe::egui::{self, Id, Rect};
use game_core3::{
    HateNum,
    buttle_char::ButtleChar,
    buttle_enemy::{
        ButtleEnemy,
        static_datas::{StaticEnemy, StaticEnemyTrait},
    },
    lt_common::LtCommon,
    skill::{SkillTrait, SkillWithState},
    static_char::StaticCharId,
};
// use game_core::{
//     chars::StaticCharId,
//     lt::{Char, Enemy, LtCommon},
//     skills::ActiveSkillState,
// };
use itertools::Itertools;

// use crate::{
//     CustomContext,
//     text::{rich_small_text, rich_title, rich_txt},
// };

#[derive(Debug, Clone, Copy)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    fn assign_to(&self, ui: &mut egui::Ui) {
        ui.set_width(self.width);
        ui.set_height(self.height);
        ui.set_max_width(self.width);
        ui.set_max_height(self.height);
    }
}

pub fn draw_game_screen(ui: &mut egui::Ui, screen_rect: Rect, state: &mut GamePageState) {
    state.receiver.update();

    let screen_width = screen_rect.width() - 100.0;
    let screen_height = screen_rect.height() - 25.0;

    // ctx.ctx.gap
    ui.vertical(|ui| {
        ui.horizontal_top(|ui| {
            let top_side_height = screen_height / 2.0;
            // log
            ui.group(|ui| {
                let size = Size {
                    height: top_side_height,
                    width: screen_width * 0.2,
                };
                size.assign_to(ui);
                draw_log(ui, state, size);
            });
            // enemy
            ui.group(|ui| {
                let enemy_item_size = Size {
                    height: top_side_height,
                    width: screen_width * 0.7,
                };

                enemy_item_size.assign_to(ui);

                draw_enemy_side(ui, enemy_item_size, state);
            });

            ui.group(|ui| {
                let size = Size {
                    height: top_side_height,
                    width: screen_width * 0.1,
                };
                size.assign_to(ui);
                draw_char_hate(ui, state, size);
            });
        });

        // player
        ui.horizontal_top(|ui| {
            let size = Size {
                height: screen_height / 2.0,
                width: screen_width,
            };

            size.assign_to(ui);
            draw_player_side(ui, state, size);
        });
    });
}

fn draw_enemy_item(ui: &mut egui::Ui, size: Size, page_state: &GamePageState, enemy: &ButtleEnemy) {
    size.assign_to(ui);
    ui.vertical(|ui| {
        ui.horizontal_top(|ui| {
            let itemsize = Size {
                width: size.width * 0.9,
                height: size.height * 0.3,
            };
            draw_lt_status(ui, itemsize, enemy.lt(), enemy.static_data().name());
        });

        ui.label(rich_txt(format!("SP: {}", enemy.sp())));

        for passive in enemy.lt().passive.display_passives() {
            ui.label(rich_txt(passive.header));
        }

        let focused = page_state
            .receiver
            .state()
            .focused_enemy()
            .map(|e| e.runtime_id())
            == Some(enemy.runtime_id());

        let btn_msg = if focused {
            "選択中"
        } else {
            "ターゲットに選択"
        };

        if ui.button(rich_txt(btn_msg)).clicked() {
            page_state
                .sender()
                .change_focus(enemy.runtime_id())
                .unwrap();
        }
    });
}

fn draw_enemy_side(ui: &mut egui::Ui, size: Size, page_state: &GamePageState) {
    size.assign_to(ui);
    ui.horizontal_top(|ui| {
        for enemy in page_state.game_state().enemys().current_wave_enemys() {
            let item_size = Size {
                height: size.height,
                width: size.width * 0.3,
            };

            ui.group(|ui| {
                draw_enemy_item(ui, item_size, page_state, enemy);
            });
        }
    });
}

// fn draw_enemy_item(ui: &mut egui::Ui, ctx: &CustomContext<'_>, size: Size, enemy: &Enemy) {
// }

// fn draw_enemy_side(ui: &mut egui::Ui, page_state: &mut GamePageState, size: Size) {
//     let enemy = page_state.game_state().enemys();
//     // draw_enemy_item(ui, ctx, size, enemy);
// }

fn draw_char_item(ui: &mut egui::Ui, page_state: &GamePageState, size: Size, char: &ButtleChar) {
    ui.group(|ui| {
        ui.vertical(|ui| {
            size.assign_to(ui);

            ui.horizontal_top(|ui| {
                let icon_width = size.width * 0.3;
                // ui.add(egui::Image::new(&tex_handle).max_width(icon_width));
                let status_size = Size {
                    height: icon_width,
                    width: size.width - icon_width,
                };
                draw_lt_status(ui, status_size, char.lt(), char.static_data().name);
            });

            egui::Frame::none().show(ui, |ui| {
                let skill_item_size = Size {
                    width: size.width / 2.0,
                    height: size.height * 0.15,
                };

                for skills in char.skills.skills().chunks(2) {
                    ui.horizontal_top(|ui| {
                        for skill in skills {
                            draw_skill_item(
                                ui,
                                page_state,
                                skill_item_size,
                                char.static_data().id,
                                skill,
                            );
                        }
                    });
                }

                for passive in char.lt().passive.display_passives() {
                    ui.label(rich_txt(passive.header));
                }
            });
        });
    });
}

fn draw_skill_item(
    ui: &mut egui::Ui,
    page_state: &GamePageState,
    size: Size,
    user_id: StaticCharId,
    skill: &SkillWithState,
) {
    ui.group(|ui| {
        size.assign_to(ui);
        ui.vertical(|ui| {
            ui.label(rich_txt(skill.static_data().document().name));
            ui.label(rich_small_text(format!("CT: {}", skill.cooldown())));
            ui.label(rich_txt(format!(
                "必要MP: {}",
                skill.need_mp(
                    page_state
                        .game_state()
                        .chars()
                        .get_char_by_static_id(user_id)
                        .unwrap(),
                    page_state.game_state()
                )
            )));
            ui.horizontal_top(|ui| {
                let useable = skill.useable(
                    page_state
                        .game_state()
                        .chars()
                        .get_char_by_static_id(user_id)
                        .unwrap(),
                    page_state.game_state(),
                );

                let msg = if useable { "使用" } else { "使用不可" };
                let btn = ui.button(rich_txt(msg));

                if useable && btn.clicked() {
                    page_state
                        .sender()
                        .use_skill(user_id, skill.static_data().id())
                        .unwrap();
                }

                // padding
                egui::Frame::none().show(ui, |ui| {
                    ui.set_min_width(size.width * 0.04);
                });

                // let open_window = ctx
                //     .ui_state
                //     .get_open_skill_window(user_id, skill.static_data.id);

                // let open_window_btn_msg = if open_window { "閉じる" } else { "詳細" };
                // if ui.button(rich_txt(open_window_btn_msg)).clicked() {
                //     ctx.ui_state
                //         .set_open_skill_window(user_id, skill.static_data.id, !open_window);
                // }

                // if open_window {
                //     egui::Window::new(rich_txt(skill.static_data.name))
                //         .id(Id::new((user_id, skill.static_data.id)))
                //         .show(ctx.ctx, |ui| {
                //             ui.label(rich_txt(skill.static_data.text));
                //             if ui.button(rich_txt("閉じる")).clicked() {
                //                 ctx.ui_state.set_open_skill_window(
                //                     user_id,
                //                     skill.static_data.id,
                //                     false,
                //                 );
                //             }
                //         });
                // }
            });
        });
    });
}

fn draw_player_side(ui: &mut egui::Ui, page_state: &mut GamePageState, size: Size) {
    ui.horizontal_top(|ui| {
        let item_size = Size {
            height: size.height,
            width: size.width / 5.2,
        };
        egui::Frame::none().show(ui, |ui| {
            draw_player_panel(ui, page_state, item_size);
        });

        for char in page_state.game_state().chars().chars() {
            egui::Frame::none().show(ui, |ui| {
                draw_char_item(ui, page_state, item_size, char);
            });
        }
    });
}

fn draw_player_panel(ui: &mut egui::Ui, page_state: &GamePageState, size: Size) {
    ui.group(|ui| {
        ui.vertical(|ui| {
            size.assign_to(ui);
            ui.label(rich_txt(format!(
                "MP: {}",
                page_state.game_state().player_mp()
            )));
            if ui.button(rich_txt("ターンエンド")).clicked() {
                page_state.sender().turnend().unwrap();
            }
        });
    });
}

fn draw_log(ui: &mut egui::Ui, page_state: &GamePageState, _size: Size) {
    ui.vertical(|ui| {
        // ui.label(txt("ログ"));
        egui::ScrollArea::vertical().show(ui, |ui| {
            for l in page_state.logs() {
                ui.label(rich_txt(l));
            }
        });
    });
}

fn draw_char_hate(ui: &mut egui::Ui, page_state: &GamePageState, size: Size) {
    ui.vertical(|ui| {
        size.assign_to(ui);
        ui.label(rich_txt("ヘイト順"));

        let mut chars = page_state
            .game_state()
            .chars()
            .chars()
            .iter()
            .collect::<Vec<_>>();

        chars.sort_by_key(|char| HateNum::MAX - char.hate());

        debug_assert!({
            // キャラクターは1体以上いる
            // 配列の最初のキャラクターのヘイトが最後のキャラクターのヘイト以上である
            let f = chars.first().unwrap();
            let l = chars.last().unwrap();
            f.hate() >= l.hate()
        });

        for char in chars {
            ui.group(|ui| {
                ui.set_width(size.width * 0.8);
                ui.label(rich_txt(char.static_data().name));
                ui.label(rich_txt(format!("hate: {}", char.hate())));
            });
        }
    });
}

fn draw_lt_status(ui: &mut egui::Ui, size: Size, lt: &LtCommon, name: &str) {
    ui.vertical(|ui| {
        size.assign_to(ui);

        ui.horizontal_top(|ui| {
            // padding
            egui::Frame::none().show(ui, |ui| {
                ui.set_min_width(size.width * 0.05);
            });
            ui.label(rich_title(format!("{} lv.{}", name, lt.level())));
        });

        ui.horizontal_top(|ui| {
            // padding
            egui::Frame::none().show(ui, |ui| {
                ui.set_min_width(size.width * 0.05);
            });
            ui.vertical(|ui| {
                ui.label(rich_small_text(format!(
                    "HP: {}/{}",
                    lt.hp().round() as u32,
                    lt.max_hp().round() as u32
                )));
                ui.label(rich_small_text(format!(
                    "物理攻撃力: {}",
                    lt.physics_attuck().round()
                )));
                ui.label(rich_small_text(format!(
                    "魔法攻撃力: {}",
                    lt.magic_attuck().round()
                )));
                ui.label(rich_small_text(format!(
                    "被物理ダメージ倍率: {}",
                    (lt.recv_physics_dmg_mag() * 100.0).round() / 100.0
                )));
                ui.label(rich_small_text(format!(
                    "被魔法ダメージ倍率: {}",
                    (lt.recv_magic_dmg_mag() * 100.0).round() / 100.0
                )));
            });

            // padding
            egui::Frame::none().show(ui, |ui| {
                ui.set_min_width(size.width * 0.05);
            });

            ui.vertical(|ui| {
                ui.label(rich_small_text(format!("INT: {}", lt.int())));
                ui.label(rich_small_text(format!("STR: {}", lt.str())));
                ui.label(rich_small_text(format!("VIT: {}", lt.vit())));
                ui.label(rich_small_text(format!("DEX: {}", lt.dex())));
                ui.label(rich_small_text(format!("AGI: {}", lt.agi())));
            });
        });
    });
}
