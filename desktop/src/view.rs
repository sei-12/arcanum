// use game_core9::game_state::GameState;
// use iced::Element;

// use crate::Message;

// pub fn view(state: &GameState) -> Element<'_, Message> {
//    todo!()
// }

use std::cmp::max;

use game_core9::{buttle_char::ButtleChar, buttle_enemy::ButtleEnemy, lt_common::LtCommon};
use iced::{
    Background, Border, Color, Element,
    Length::{self},
    alignment::Horizontal,
    border::Radius,
    widget::{Column, Container, Row, button, column, container, row, scrollable, text},
};

use crate::{Ctx, Message, common::round_digits::RoundDigits};

pub fn game_view<'a>(ctx: Ctx<'a>) -> Container<'a, Message> {
    container(column![
        enemy_side_view(ctx).into(),
        player_side_view(ctx).into()
    ])
}

fn enemy_side_view<'a>(ctx: Ctx<'a>) -> impl Into<Element<'a, Message>> {
    let state = ctx.game_state;
    column![enemy_item_view(state.get_enemy(), ctx).into()]
        .width(Length::Fill)
        .align_x(Horizontal::Center)
}

fn enemy_item_view<'a>(enemy: &'a ButtleEnemy, ctx: Ctx<'a>) -> impl Into<Element<'a, Message>> {
    let enemy_name = enemy.info().name;
    column![
        text(enemy_name).size(23),
        lt_common_view(enemy.lt(), ctx).into()
    ]
    .padding(20)
}

fn player_side_view<'a>(ctx: Ctx<'a>) -> impl Into<Element<'a, Message>> {
    let state = ctx.game_state;
    row![
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
    let char_name = char.name();
    container(
        row![column![
            text(char_name).size(30),
            lt_common_view(char.lt(), ctx).into()
        ],]
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

fn lt_common_view<'a>(lt_common: &LtCommon, _ctx: Ctx<'a>) -> impl Into<Element<'a, Message>> {
    column![
        row![
            column![
                row![text!(
                    "HP: {}/{}",
                    lt_common.hp().round(),
                    lt_common.max_hp().round()
                )],
                bar(
                    lt_common.hp(),
                    lt_common.max_hp(),
                    Color::from_rgb8(0, 255, 255)
                )
                .width(Length::FillPortion(1)),
                row![text!(
                    "MP: {}/{}",
                    lt_common.mp().round(),
                    lt_common.max_mp().round()
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

fn bar(val: f32, max_val: f32, color: Color) -> Container<'static, Message> {
    assert!(val <= max_val);

    let inner_width = (val / max_val * 100.0).round() as u16;
    dbg!(inner_width);

    let mut inner = row![
        container(text(""))
            .style(move |_t| container::Style {
                background: Some(Background::Color(color)),
                ..Default::default()
            })
            .width(Length::FillPortion(inner_width)),
    ];

    if inner_width != 100 {
        inner = inner.push(container(text("")).width(Length::FillPortion(100 - inner_width)))
    }

    container(inner).style(move |_t| container::Style {
        background: Some(Background::Color(Color::BLACK)),
        border: Border {
            color: Color::from_rgb(20.0, 20.0, 20.0),
            width: 1.0,
            radius: Radius::new(10),
        },
        ..Default::default()
    })
}
