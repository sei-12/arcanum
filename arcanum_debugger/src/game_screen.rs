use eframe::egui::{self, Id};
use game_core::{
    chars::StaticCharId,
    lt::{Char, Enemy, LtCommon},
    skills::ActiveSkillState,
};
use itertools::Itertools;

use crate::{
    CustomContext,
    text::{rich_small_text, rich_title, rich_txt},
};

#[derive(Debug, Clone, Copy)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    fn assign_to(&self, ui: &mut egui::Ui) {
        ui.set_max_width(self.width);
        ui.set_max_height(self.height);
        ui.set_width(self.width);
        ui.set_height(self.height);
    }
}

pub fn draw_game_screen(ui: &mut egui::Ui, ctx: &CustomContext<'_>) {
    let screen_width = ctx.ctx.screen_rect().width() - 100.0;
    let screen_height = ctx.ctx.screen_rect().height() - 25.0;

    // ctx.ctx.gap
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            let top_side_height = screen_height / 2.0;
            // log
            ui.group(|ui| {
                let size = Size {
                    height: top_side_height,
                    width: screen_width * 0.2,
                };
                size.assign_to(ui);
                draw_log(ui, ctx, size);
            });
            // enemy
            ui.group(|ui| {
                let enemy_item_size = Size {
                    height: top_side_height,
                    width: screen_width * 0.7,
                };

                enemy_item_size.assign_to(ui);

                draw_enemy_side(ui, ctx, enemy_item_size);
            });

            ui.group(|ui| {
                let size = Size {
                    height: top_side_height,
                    width: screen_width * 0.1,
                };
                size.assign_to(ui);
                draw_char_hate(ui, ctx, size);
            });
        });

        // player
        ui.horizontal_top(|ui| {
            let size = Size {
                height: screen_height / 2.0,
                width: screen_width,
            };

            size.assign_to(ui);
            draw_player_side(ui, ctx, size);
        });
    });
}

fn draw_enemy_item(ui: &mut egui::Ui, ctx: &CustomContext<'_>, size: Size, enemy: &Enemy) {
    size.assign_to(ui);
    ui.vertical_centered(|ui| {
        size.assign_to(ui);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let icon = ctx.img_loader.get_enemy_icon(enemy.static_data.id);
                let tex_handle = ctx
                    .ctx
                    .load_texture("1", icon, egui::TextureOptions::default());
                ui.add(egui::Image::new(&tex_handle).max_width(size.width * 0.3));

                let status_size = Size {
                    width: size.width * 0.2,
                    height: size.width * 0.1,
                };
                draw_lt_status(ui, ctx, status_size, enemy, enemy.static_data.name);
            });

            egui::Frame::none().show(ui, |ui| {
                let enemy_actions_item_size = Size {
                    height: size.height * 0.05,
                    width: size.width * 0.9,
                };

                enemy_actions_item_size.assign_to(ui);
                draw_enemy_actions(ui, ctx, enemy_actions_item_size);
            });

            for passive in enemy.passive.displayble_passives() {
                ui.label(rich_txt(&passive));
            }
        });
    });
}

fn draw_enemy_side(ui: &mut egui::Ui, ctx: &CustomContext<'_>, size: Size) {
    let enemy = &ctx.core.get_state().enemy;
    draw_enemy_item(ui, ctx, size, enemy);
}

fn draw_char_item(ui: &mut egui::Ui, ctx: &CustomContext<'_>, size: Size, char: &Char) {
    ui.group(|ui| {
        ui.vertical(|ui| {
            size.assign_to(ui);
            let char_icon = ctx.img_loader.get_char_icon(char.static_data.id);
            let tex_handle = ctx
                .ctx
                .load_texture("1", char_icon, egui::TextureOptions::default());

            ui.horizontal(|ui| {
                let icon_width = size.width * 0.3;
                ui.add(egui::Image::new(&tex_handle).max_width(icon_width));
                let status_size = Size {
                    height: icon_width,
                    width: size.width - icon_width,
                };
                draw_lt_status(ui, ctx, status_size, char, char.static_data.name);
            });

            egui::Frame::none().show(ui, |ui| {
                let skill_item_size = Size {
                    width: size.width / 2.0,
                    height: size.height / 10.0,
                };

                for skills in &char.skills.iter().chunks(2) {
                    ui.horizontal(|ui| {
                        for skill in skills {
                            draw_skill_item(ui, ctx, skill_item_size, char.static_data.id, skill);
                        }
                    });
                }
                for passive in char.passive.displayble_passives() {
                    ui.label(rich_txt(&passive));
                }
            });
        });
    });
}

fn draw_skill_item(
    ui: &mut egui::Ui,
    ctx: &CustomContext<'_>,
    size: Size,
    user_id: StaticCharId,
    skill: &ActiveSkillState,
) {
    ui.group(|ui| {
        size.assign_to(ui);
        ui.vertical(|ui| {
            ui.label(rich_txt(skill.static_data.name));
            ui.horizontal(|ui| {
                ui.label(rich_txt(format!(
                    "必要MP: {}",
                    skill.static_data.need_mp.round() as u32
                )));
                let useable = skill.useable(ctx.core.get_state());

                let msg = if useable { "使用" } else { "使用不可" };
                let btn = ui.button(rich_txt(msg));

                if useable && btn.clicked() {
                    ctx.core.use_skill(user_id, skill.static_data.id);
                }

                let open_window = ctx
                    .ui_state
                    .get_open_skill_window(user_id, skill.static_data.id);

                let open_window_btn_msg = if open_window { "閉じる" } else { "詳細" };
                if ui.button(rich_txt(open_window_btn_msg)).clicked() {
                    ctx.ui_state
                        .set_open_skill_window(user_id, skill.static_data.id, !open_window);
                }

                if open_window {
                    egui::Window::new(rich_txt(skill.static_data.name))
                        .id(Id::new((user_id, skill.static_data.id)))
                        .show(ctx.ctx, |ui| {
                            ui.label(rich_txt(skill.static_data.text));
                            if ui.button(rich_txt("閉じる")).clicked() {
                                ctx.ui_state.set_open_skill_window(
                                    user_id,
                                    skill.static_data.id,
                                    false,
                                );
                            }
                        });
                }
            });
        });
    });
}

fn draw_enemy_actions(ui: &mut egui::Ui, ctx: &CustomContext<'_>, size: Size) {
    ui.horizontal(|ui| {
        size.assign_to(ui);

        ui.label(rich_txt("次の行動: "));

        for action in ctx.core.get_state().enemy_actions.iter() {
            let action_txt = match *action {
                game_core::enemy_ai::EnemyAction::Assist => "補",
                game_core::enemy_ai::EnemyAction::High => "強",
                game_core::enemy_ai::EnemyAction::Interference => "妨",
                game_core::enemy_ai::EnemyAction::Low => "弱",
                game_core::enemy_ai::EnemyAction::Mid => "中",
            };

            ui.group(|ui| ui.label(rich_txt(action_txt)));
        }
    });
}

fn draw_player_side(ui: &mut egui::Ui, ctx: &CustomContext<'_>, size: Size) {
    ui.horizontal(|ui| {
        let item_size = Size {
            height: size.height,
            width: size.width / 5.2,
        };
        egui::Frame::none().show(ui, |ui| {
            draw_player_panel(ui, ctx, item_size);
        });

        for char in ctx.core.get_state().chars.iter() {
            egui::Frame::none().show(ui, |ui| {
                draw_char_item(ui, ctx, item_size, char);
            });
        }
    });
}

fn draw_player_panel(ui: &mut egui::Ui, ctx: &CustomContext<'_>, size: Size) {
    ui.group(|ui| {
        ui.vertical(|ui| {
            size.assign_to(ui);
            ui.label(rich_txt(format!(
                "MP: {}",
                ctx.core.get_state().player_side_mp.round() as u32
            )));
            if ui.button(rich_txt("ターンエンド")).clicked() {
                ctx.core.turn_end();
            }
        });
    });
}

fn draw_log(ui: &mut egui::Ui, ctx: &CustomContext<'_>, _size: Size) {
    ui.vertical(|ui| {
        // ui.label(txt("ログ"));
        egui::ScrollArea::vertical().show(ui, |ui| {
            for l in ctx.log {
                ui.label(rich_txt(l));
            }
        });
    });
}

fn draw_char_hate(ui: &mut egui::Ui, ctx: &CustomContext<'_>, size: Size) {
    ui.vertical(|ui| {
        size.assign_to(ui);
        ui.label(rich_txt("ヘイト順"));

        let mut chars = ctx.core.get_state().chars.iter().collect::<Vec<_>>();
        chars.sort_by(|a, b| b.hate.partial_cmp(&a.hate).unwrap());

        debug_assert!({
            // キャラクターは1体以上いる
            // 配列の最初のキャラクターのヘイトが最後のキャラクターのヘイト以上である
            let f = chars.first().unwrap();
            let l = chars.last().unwrap();
            f.hate >= l.hate
        });

        for char in chars {
            ui.group(|ui| {
                ui.set_width(size.width * 0.8);
                ui.label(rich_txt(char.static_data.name));
                ui.label(rich_txt(format!("hate: {}", char.hate.round() as u32)));
            });
        }
    });
}

fn draw_lt_status(
    ui: &mut egui::Ui,
    _ctx: &CustomContext<'_>,
    size: Size,
    lt: &LtCommon,
    name: &str,
) {
    ui.vertical(|ui| {
        size.assign_to(ui);

        ui.horizontal_top(|ui| {
            // padding
            egui::Frame::none().show(ui, |ui| {
                ui.set_min_width(size.width * 0.05);
            });
            ui.label(rich_title(format!("{} lv.{}", name, lt.level)));
        });

        ui.horizontal_top(|ui| {
            // padding
            egui::Frame::none().show(ui, |ui| {
                ui.set_min_width(size.width * 0.05);
            });
            ui.vertical(|ui| {
                ui.label(rich_small_text(format!(
                    "HP: {}/{}",
                    lt.hp.round() as u32,
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
                    "物理ダメージ軽減率: {}",
                    ((1.0 - lt.physics_defence()) * 100.0).round() / 100.0
                )));
                ui.label(rich_small_text(format!(
                    "魔法ダメージ軽減率: {}",
                    ((1.0 - lt.magic_defence()) * 100.0).round() / 100.0
                )));
            });

            // padding
            egui::Frame::none().show(ui, |ui| {
                ui.set_min_width(size.width * 0.05);
            });

            ui.vertical(|ui| {
                ui.label(rich_small_text(format!("INT: {}", lt.potential().int)));
                ui.label(rich_small_text(format!("STR: {}", lt.potential().str)));
                ui.label(rich_small_text(format!("VIT: {}", lt.potential().vit)));
                ui.label(rich_small_text(format!("DEX: {}", lt.potential().dex)));
                ui.label(rich_small_text(format!("AGI: {}", lt.potential().agi)));
            });
        });
    });
}
