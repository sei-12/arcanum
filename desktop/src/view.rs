use crate::{Ctx, Message, common::round_digits::RoundDigits, ui_state};
use game_core6::{
    buttle_char::{ButtleChar, ButtleSkill},
    buttle_enemy::ButtleEnemy,
    game_core_actor::GameCoreActorCommand,
    lt_common::LtCommon,
    runtime_id::RuntimeCharId,
};
use iced::{
    Border, Color, Element,
    Length::{self},
    alignment::Horizontal,
    border::Radius,
    widget::{
        Column, Container, Row, button, canvas::path::lyon_path::geom::euclid::num::Round, column,
        container, row, scrollable, text,
    },
};

pub fn game_view<'a>(ctx: Ctx<'a>) -> Container<'a, Message> {
    container(column![
        enemy_side_view(ctx).into(),
        player_side_view(ctx).into()
    ])
}

fn enemy_side_view<'a>(ctx: Ctx<'a>) -> impl Into<Element<'a, Message>> {
    let state = ctx.game_state;
    column![
        Row::with_children(
            state
                .get_current_wave_enemys()
                .iter()
                .map(|e| enemy_item_view(e, ctx).into()),
        )
        .height(Length::FillPortion(1))
    ]
    .width(Length::Fill)
    .align_x(Horizontal::Center)
}

fn enemy_item_view<'a>(enemy: &'a ButtleEnemy, ctx: Ctx<'a>) -> impl Into<Element<'a, Message>> {
    let enemy_name = enemy.static_data().name();
    column![
        text(enemy_name).size(23),
        lt_common_view(enemy.lt(), ctx).into()
    ]
    .padding(20)
}

fn player_side_view<'a>(ctx: Ctx<'a>) -> impl Into<Element<'a, Message>> {
    let state = ctx.game_state;
    row![
        player_panel_view(ctx).into(),
        column![
            Row::with_children(
                state
                    .get_chars()
                    .iter()
                    .map(|c| char_item_view(c, ctx).into())
            )
            .height(Length::FillPortion(1))
            .width(Length::Fill)
            .spacing(5)
        ]
        .align_x(Horizontal::Center)
        .width(Length::FillPortion(6))
    ]
    .width(Length::Fill)
    .padding(10)
}

fn char_item_view<'a>(char: &'a ButtleChar, ctx: Ctx<'a>) -> Container<'a, Message> {
    let char_name = char.static_data().name;
    container(
        row![
            column![
                text(char_name).size(30),
                lt_common_view(char.lt(), ctx).into()
            ],
            column![char_skills_view(char, ctx).into()]
        ]
        .spacing(10),
    )
    .height(Length::Fill)
    .style(|_t| container::Style {
        border: Border {
            color: Color::BLACK,
            width: 1.0,
            radius: Radius::new(10),
        },
        ..Default::default()
    })
    .padding(20)
    .width(Length::FillPortion(1))
}

fn char_skills_view<'a>(char: &'a ButtleChar, ctx: Ctx<'a>) -> impl Into<Element<'a, Message>> {
    scrollable(
        Column::from_iter(
            char.skills()
                .iter()
                .map(|s| char_skill_view(s, char.runtime_id(), ctx).into()),
        )
        .spacing(5),
    )
    .height(Length::Fill)
}

fn char_skill_view<'a>(
    skill: &'a ButtleSkill,
    user_id: RuntimeCharId,
    ctx: Ctx<'a>,
) -> impl Into<Element<'a, Message>> {
    let state = ctx.game_state;
    let mut column = column![text(skill.data().info().name).size(20),];

    let mut row = row!(text!("CD: {}", skill.cooldown()).size(15)).spacing(8);

    let useable = skill.useable(state);

    if useable {
        row = row.push(button("使用").on_press(Message::GameCoreMessage(
            GameCoreActorCommand::UseSkill {
                user_id,
                skill_id: skill.runtime_id(),
                target_id: None,
            },
        )));
    } else {
        row = row.push(text("使用不可").color(Color::from_rgb(0.5, 0.0, 0.0)));
    }

    let opened = ctx
        .ui_state
        .is_skill_detail_opened(user_id, skill.runtime_id());

    if opened {
        row = row.push(button("閉じる").on_press(Message::UiStateUpdateMessage(
            ui_state::UiStateUpdateMessage::CloseSkillDetail {
                char_id: user_id,
                skill_id: skill.runtime_id(),
            },
        )))
    } else {
        row = row.push(button("▽").on_press(Message::UiStateUpdateMessage(
            ui_state::UiStateUpdateMessage::OpenSkillDetail {
                char_id: user_id,
                skill_id: skill.runtime_id(),
            },
        )))
    }

    column = column.push(column![row].align_x(Horizontal::Right).width(Length::Fill));

    if opened {
        column = column.push(
            column![
                text!("必要MP: {}", skill.data().info().default_need_mp),
                text!("クールダウン: {}", skill.data().info().defalut_cooldown),
                text!("ヘイト値: {}", skill.data().info().defalut_hate),
                text!("効果: {}", skill.data().info().description)
            ]
            .padding(5),
        );
    }

    container(column)
        .style(|_t| container::Style {
            border: Border {
                color: Color::BLACK,
                width: 1.0,
                radius: Radius::new(10),
            },
            ..Default::default()
        })
        .width(Length::Fill)
        .padding(10)
}

fn lt_common_view<'a>(lt_common: &LtCommon, _ctx: Ctx<'a>) -> impl Into<Element<'a, Message>> {
    column![
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
                    text(lt_common.recv_magic_dmg_mag().round_digits(2))
                ],
                row![
                    text("被物理ダメージ倍率: "),
                    text(lt_common.recv_physics_dmg_mag().round_digits(2))
                ],
            ],
            row![
                column![
                    row![text("INT")],
                    row![text("AGI")],
                    row![text("STR")],
                    row![text("DEX")],
                    row![text("VIT")],
                ],
                column![
                    row![text(lt_common.int().round())],
                    row![text(lt_common.agi().round())],
                    row![text(lt_common.str().round())],
                    row![text(lt_common.dex().round())],
                    row![text(lt_common.vit().round())],
                ]
            ]
            .spacing(5),
        ]
        .spacing(10),
        Row::with_children(lt_common.passive.display().map(|d| text(d).into()))
            .spacing(5)
            .padding(5)
    ]
}

fn player_panel_view<'a>(ctx: Ctx<'a>) -> impl Into<Element<'a, Message>> {
    container(
        column![
            text!("MP: {}", ctx.game_state.player_mp().round()),
            button("ターンエンド")
                .on_press(Message::GameCoreMessage(GameCoreActorCommand::TurnEnd)),
        ]
        .width(Length::FillPortion(1))
        .padding(7),
    )
}
