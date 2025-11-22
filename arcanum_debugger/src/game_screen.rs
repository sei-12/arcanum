use eframe::egui::{self};
use game_core::{chars::StaticCharId, lt::Char, skills::ActiveSkillState};
use itertools::Itertools;

use crate::{CustomContext, text::txt};

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
    // ctx.ctx.gap
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            let top_side_height = ctx.ctx.screen_rect().height() / 2.0;
            // log
            ui.group(|ui| {
                let size = Size {
                    height: top_side_height,
                    width: ctx.ctx.screen_rect().width() * 0.2,
                };
                size.assign_to(ui);
                draw_log(ui, ctx, size);
            });
            // enemy
            egui::Frame::none().show(ui, |ui| {
                let enemy_item_size = Size {
                    height: top_side_height,
                    width: ctx.ctx.screen_rect().width() * 0.6,
                };

                enemy_item_size.assign_to(ui);

                draw_enemy_side(ui, ctx, enemy_item_size);
            });

            ui.group(|ui| {
                let size = Size {
                    height: top_side_height,
                    width: ctx.ctx.screen_rect().width() * 0.1,
                };
                size.assign_to(ui);
                draw_char_hate(ui, ctx, size);
            });
        });

        // player
        egui::Frame::none().show(ui, |ui| {
            let size = Size {
                height: (ctx.ctx.screen_rect().height() / 2.0),
                width: ctx.ctx.screen_rect().width() * 0.96,
            };

            size.assign_to(ui);
            draw_player_side(ui, ctx, size);
        });
    });
}

fn draw_enemy_side(ui: &mut egui::Ui, ctx: &CustomContext<'_>, size: Size) {
    ui.horizontal_top(|ui| {
        let enemy = &ctx.core.get_state().enemy;
        let item_size = Size {
            width: size.width * 0.5,
            height: size.height,
        };

        // padding
        egui::Frame::none().show(ui, |ui| {
            let width = (size.width - item_size.width) / 2.0;
            ui.set_width(width);
        });

        ui.group(|ui| {
            item_size.assign_to(ui);
            ui.vertical(|ui| {
                ui.label(txt(enemy.static_data.name));

                let icon = ctx.img_loader.get_enemy_icon(enemy.static_data.id);
                let tex_handle = ctx
                    .ctx
                    .load_texture("1", icon, egui::TextureOptions::default());
                ui.add(egui::Image::new(&tex_handle).max_width(item_size.width / 2.5));

                ui.label(txt(&format!(
                    "HP: {}/{}",
                    enemy.hp.round() as u32,
                    enemy.max_hp() as u32
                )));

                for passive in enemy.passive.displayble_passives() {
                    ui.label(txt(&passive));
                }
            });
        });
    });
}

fn draw_char_item(ui: &mut egui::Ui, ctx: &CustomContext<'_>, size: Size, char: &Char) {
    ui.group(|ui| {
        ui.vertical(|ui| {
            size.assign_to(ui);
            ui.label(txt(char.static_data.name));
            let char_icon = ctx.img_loader.get_char_icon(char.static_data.id);
            let tex_handle = ctx
                .ctx
                .load_texture("1", char_icon, egui::TextureOptions::default());
            ui.add(egui::Image::new(&tex_handle).max_width(size.width / 2.5));
            ui.label(txt(&format!(
                "HP: {}/{}",
                char.hp.round() as u32,
                char.max_hp() as u32
            )));

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
                    ui.label(txt(&passive));
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
            ui.label(txt(skill.static_data.name));
            ui.horizontal(|ui| {
                ui.label(txt(&format!(
                    "必要MP: {}",
                    skill.static_data.need_mp.round() as u32
                )));
                let useable = skill.useable(ctx.core.get_state());

                let msg = if useable { "使用" } else { "使用不可" };
                let btn = ui.button(txt(msg));

                if useable && btn.clicked() {
                    ctx.core.use_skill(user_id, skill.static_data.id);
                }
            });
        });
    });
}

fn draw_player_side(ui: &mut egui::Ui, ctx: &CustomContext<'_>, size: Size) {
    ui.horizontal(|ui| {
        let item_size = Size {
            height: size.height - 16.0,
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
            ui.label(txt(&format!(
                "MP: {}",
                ctx.core.get_state().player_side_mp.round() as u32
            )));
            if ui.button(txt("ターンエンド")).clicked() {
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
                ui.label(txt(l));
            }
        });
    });
}

fn draw_char_hate(ui: &mut egui::Ui, ctx: &CustomContext<'_>, size: Size) {
    ui.vertical(|ui| {
        size.assign_to(ui);
        ui.label(txt("ヘイト順"));

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
                ui.label(txt(char.static_data.name));
                ui.label(txt(&format!("hate: {}", char.hate.round() as u32)));
            });
        }
    });
}
