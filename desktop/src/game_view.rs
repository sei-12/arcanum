use std::{borrow::Cow, collections::VecDeque, fmt::Display, time::Duration};

use game_core9::{
    buttle_char::{ButtleChar, CharCondition},
    buttle_enemy::ButtleEnemy,
    buttle_skill::ButtleSkill,
    core_actor::{GameCoreActor, GameCoreOutput, UserInput},
    lt_common::LtCommon,
    runtime_id::RuntimeSkillId,
};
use iced::{
    Background, Border, Color, Element, Event, Length, Padding,
    alignment::{
        Horizontal,
        Vertical::{self},
    },
    border::Radius,
    event::{self, Status},
    keyboard::{Event::KeyPressed, Key, key::Named},
    widget::{
        Button, Column, Container, Image, Row, button, column, container, pick_list, row,
        scrollable, text, toggler, tooltip,
    },
};

use crate::{
    HomePage, MainAppMessage, PageTrait, assets::game_core, common::round_digits::RoundDigits,
};

#[derive(Debug, Clone)]
pub enum GameViewMessage {
    GameLoopTick,
    Paused(bool),
    OnPressSpace,
    UpdateGameSpeed(GameSpeed),
    UseSkill(RuntimeSkillId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameSpeed {
    // Pause,
    Slow,
    Half,
    Normal,
    Double,
}
impl GameSpeed {
    fn fps(&self) -> u8 {
        match self {
            GameSpeed::Slow => 100,
            GameSpeed::Half => 20,
            GameSpeed::Normal => 10,
            GameSpeed::Double => 5,
        }
    }
}

impl Display for GameSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameSpeed::Slow => f.write_str("0.1倍速"),
            GameSpeed::Half => f.write_str("0.5倍速"),
            GameSpeed::Normal => f.write_str("1倍速"),
            GameSpeed::Double => f.write_str("2倍速"),
        }
    }
}

#[derive(Debug)]
pub struct GamePage {
    paused: bool,
    speed: GameSpeed,
    core: GameCoreActor,
    use_skill_queue: VecDeque<RuntimeSkillId>,
}

impl GamePage {
    pub(crate) fn new() -> Self {
        Self {
            speed: GameSpeed::Normal,
            paused: false,
            core: game_core(),
            use_skill_queue: VecDeque::new(),
        }
    }

    fn game_loop_tick(&mut self) -> Option<Box<dyn PageTrait>> {
        let user_input = match self.use_skill_queue.pop_front() {
            Some(id) => UserInput::UseSkill { skill_id: id },
            None => UserInput::None,
        };

        let mut output_buffer = Vec::new();
        self.core.tick(user_input, &mut output_buffer).unwrap();

        for output in output_buffer {
            if let GameCoreOutput::Event(_) = output {
                return Some(Box::new(HomePage));
            }
        }

        None
    }
}

impl PageTrait for GamePage {
    fn update(&mut self, msg: MainAppMessage) -> Option<Box<dyn PageTrait>> {
        let MainAppMessage::GameViewMessage(msg) = msg else {
            return None;
        };

        match msg {
            GameViewMessage::GameLoopTick => self.game_loop_tick(),
            GameViewMessage::Paused(p) => {
                self.paused = p;
                None
            }
            GameViewMessage::OnPressSpace => {
                self.paused = !self.paused;
                None
            }
            GameViewMessage::UpdateGameSpeed(spd) => {
                self.speed = spd;
                None
            }
            GameViewMessage::UseSkill(runtime_skill_id) => {
                self.use_skill_queue.push_back(runtime_skill_id);
                None
            }
        }
    }

    fn view(&self) -> Element<'_, MainAppMessage> {
        column![
            column![
                row![
                    self.hate_list_view(),
                    self.enemy_side_view().width(Length::Fill),
                ]
                .height(Length::Fill),
            ]
            .align_x(Horizontal::Center)
            .height(Length::Fill)
            .width(Length::Fill),
            self.player_side_view()
        ]
        .spacing(5)
        .padding(10)
        .into()
    }

    fn subscription(&self, subscriptions: &mut Vec<iced::Subscription<MainAppMessage>>) {
        if !self.paused {
            let fps = self.speed.fps();
            subscriptions.push(
                iced::time::every(Duration::from_millis(fps as u64))
                    .map(|_| MainAppMessage::GameViewMessage(GameViewMessage::GameLoopTick)),
            );
        }

        let s = event::listen_with(move |event, status, _| match (event, status) {
            (
                Event::Keyboard(KeyPressed {
                    key: Key::Named(Named::Space),
                    ..
                }),
                Status::Ignored,
            ) => Some(MainAppMessage::GameViewMessage(
                GameViewMessage::OnPressSpace,
            )),
            _ => None,
        });

        subscriptions.push(s);
    }
}

//--------------------------------------------------//
//                                                  //
//                       VIEW                       //
//                                                  //
//--------------------------------------------------//
impl GamePage {
    fn enemy_side_view(&self) -> Column<'_, MainAppMessage> {
        column![row![self.enemy_item_view(self.core.state().get_enemy())].spacing(10)]
            .align_x(Horizontal::Center)
    }

    fn player_side_view(&self) -> Row<'_, MainAppMessage> {
        row![
            self.auther_panel(),
            column![
                Row::with_children(
                    self.core
                        .state()
                        .get_chars()
                        .iter()
                        .map(|c| self.char_item_view(c).into())
                )
                .spacing(5)
            ]
            .align_x(Horizontal::Center)
        ]
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill)
    }

    fn hate_list_view(&self) -> Column<'_, MainAppMessage> {
        let chars = self.core.state().get_chars_sorted_by_hate();
        column![
            text!("ヘイト順").size(20),
            scrollable(
                Column::with_children(chars.iter().map(|c| self.hate_list_item(c)),).spacing(5)
            )
        ]
        .padding(10)
        .spacing(5)
    }

    fn hate_list_item(&self, char: &ButtleChar) -> Element<'_, MainAppMessage> {
        text!("{} ({})", char.name(), char.hate().round()).into()
    }

    fn enemy_item_view<'a>(&'a self, enemy: &'a ButtleEnemy) -> Container<'a, MainAppMessage> {
        container(self.lt_view(enemy.into()))
            .height(Length::Fill)
            .padding(10)
            .align_x(Horizontal::Center)
            .style(|_t| container::Style {
                border: Border {
                    color: Color::BLACK,
                    width: 1.0,
                    radius: Radius::new(10),
                },
                ..Default::default()
            })
    }

    fn char_item_view<'a>(&'a self, char: &'a ButtleChar) -> Container<'a, MainAppMessage> {
        container(column![self.lt_view(char.into()), self.char_skills(char)].spacing(10))
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

    fn auther_panel(&self) -> iced::widget::Column<'_, MainAppMessage> {
        let pick_l = pick_list(
            vec![
                GameSpeed::Slow,
                GameSpeed::Half,
                GameSpeed::Normal,
                GameSpeed::Double,
            ],
            Some(self.speed),
            |a| MainAppMessage::GameViewMessage(GameViewMessage::UpdateGameSpeed(a)),
        );

        column![
            pick_l,
            toggler(self.paused)
                .label("一時停止")
                .on_toggle(|v| MainAppMessage::GameViewMessage(GameViewMessage::Paused(v)))
        ]
    }

    fn lt_view<'a>(&'a self, lt: Lt<'a>) -> Row<'a, MainAppMessage> {
        let name = lt.name();
        let icon_size = 130.0;
        let image = Image::new(lt.icon_src())
            .width(Length::Fixed(icon_size))
            .height(Length::Fixed(icon_size));

        row![
            column![
                container(text(name).size(25)).padding(5),
                row![
                    image,
                    row![
                        column![
                            text!(
                                "HP: {}/{}",
                                lt.lt_common().hp().round(),
                                lt.lt_common().max_hp().round()
                            ),
                            bar(
                                lt.lt_common().hp(),
                                lt.lt_common().max_hp(),
                                Color::from_rgb8(0x00, 0xE9, 0x35)
                            )
                            .width(Length::Fill)
                            .height(Length::Fixed(11.0)),
                            text!(
                                "MP: {}/{}",
                                lt.lt_common().mp().round(),
                                lt.lt_common().max_mp().round()
                            ),
                            bar(
                                lt.lt_common().mp(),
                                lt.lt_common().max_mp(),
                                Color::from_rgb8(0x00, 0xFF, 0xFF)
                            )
                            .width(Length::Fill)
                            .height(Length::Fixed(11.0)),
                            column![row![tooltip(
                                button(text!("ステータス詳細").size(14)).padding(3),
                                container(self.lt_status_view(lt.lt_common()))
                                    .style(move |_| {
                                        container::Style {
                                            background: Some(Background::Color(Color::WHITE)),
                                            text_color: Some(Color::BLACK),
                                            border: Border {
                                                color: Color::BLACK,
                                                width: 1.0,
                                                radius: Radius::new(0),
                                            },
                                            ..Default::default()
                                        }
                                    })
                                    .padding(5),
                                tooltip::Position::Top
                            )],]
                            .align_x(Horizontal::Right)
                            .width(Length::Fill)
                            .padding(Padding::new(0.0).top(5)),
                        ]
                        .padding(10)
                        .spacing(5),
                    ]
                ]
                .align_y(Vertical::Bottom),
                column![text(lt.condition())]
                    .width(Length::Fill)
                    .align_x(Horizontal::Center),
                self.passive_list(lt)
            ]
            .spacing(7),
        ]
        .spacing(10)
    }

    fn passive_list(&self, lt: Lt<'_>) -> Container<'_, MainAppMessage> {
        // https://github.com/iced-rs/iced/discussions/2013
        // ここで議論されているようなUIにしたいけどやり方がわからん
        // 親要素のサイズを求める方法がわからん

        let mut col = Column::new();

        for passive_text in lt.lt_common().passive.display() {
            col = col.push(text(passive_text));
        }

        Container::new(scrollable(col.width(Length::Fill)).width(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fixed(50.0))
    }

    fn char_skills(&self, char: &ButtleChar) -> Column<'_, MainAppMessage> {
        Column::with_children(char.get_skills().chunks(2).map(|row_skills| {
            let mut r = Row::with_children(row_skills.iter().map(|s| self.skill_view(s).into()));

            if row_skills.len() == 1 {
                r = r.push(iced::widget::Space::with_width(Length::FillPortion(1)))
            }

            r.into()
        }))
    }

    fn skill_view(&self, skill: &ButtleSkill) -> Container<'_, MainAppMessage> {
        container(
            column![
                text(skill.skill_box().info().name).size(18),
                row![
                    column![
                        row![
                            button(text!("詳細").size(14)).padding(3),
                            self.skill_use_button(skill)
                        ]
                        .spacing(5)
                    ]
                    .align_x(Horizontal::Right)
                    .width(Length::Fill),
                ]
                .width(Length::Fill),
            ]
            .spacing(5),
        )
        .padding(10)
        .width(Length::FillPortion(1))
        .style(|_t| container::Style {
            border: Border {
                color: Color::BLACK,
                width: 1.0,
                radius: Radius::new(5),
            },
            ..Default::default()
        })
    }

    fn skill_use_button(&self, skill: &ButtleSkill) -> Button<'_, MainAppMessage> {
        let useable = skill.useable(self.core.state());

        let mut button = button(text("使用").size(14)).padding(3);

        if useable {
            button = button.on_press(MainAppMessage::GameViewMessage(GameViewMessage::UseSkill(
                skill.runtime_id(),
            )))
        }

        button
    }

    fn lt_status_view(&self, lt_common: &LtCommon) -> Column<'_, MainAppMessage> {
        column![
            row![
                column![

                    row![
                        tooltip(
                            text("最大HP: "),
                            pop_up_tooltip(
                                "最大HP = (VIT * 6 + STR + DEX) / 8 * (level + 10) * 3 * 最大HP倍率"
                            ),
                            tooltip::Position::Top
                        ),
                        tooltip(
                            text(lt_common.max_hp().round()),
                            pop_up_tooltip(format!("{}", lt_common.max_hp().round_digits(10))),
                            tooltip::Position::Top
                        ),
                    ],
                    row![
                        tooltip(
                            text("最大MP: "),
                            pop_up_tooltip(
                                "最大MP = (VIT * 2 + INT + DEX) / 4 * 50 * 最大MP倍率"
                            ),
                            tooltip::Position::Top
                        ),
                        text(lt_common.max_mp().round())
                    ],
                    row![
                        tooltip(
                            text("魔法攻撃力: "),
                            pop_up_tooltip(
                                "魔法攻撃力 = ((INT * 3 + DEX) / 4 * (level + 10) + 武器魔法攻撃力) * 魔法攻撃力倍率"
                            ),
                            tooltip::Position::Top
                        ),
                        text(lt_common.magic_attuck().round())
                    ],
                    row![
                        tooltip(
                            text("物理攻撃力: "),
                            pop_up_tooltip(
                                "物理攻撃力 = ((STR * 3 + DEX) / 4 * (level + 10) + 武器物理攻撃力) * 物理攻撃力倍率"
                            ),
                            tooltip::Position::Top
                        ),
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
                    row![
                        tooltip(
                            text("MP回復力: "),
                            pop_up_tooltip(
                                "MP回復力がNの時、MPが1秒あたりN回復します"
                            ),
                            tooltip::Position::Top
                        ),
                        text((lt_common.mp_heal() * 100.0).round_digits(2))
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
}

fn pop_up_tooltip<'a>(txt: impl Into<Cow<'a, str>>) -> Container<'a, MainAppMessage> {
    let txt = txt.into();
    container(text(txt).size(12))
        .padding(5)
        .style(move |_| container::Style {
            background: Some(Background::Color(Color::WHITE)),
            text_color: Some(Color::BLACK),
            ..Default::default()
        })
}

fn bar(val: f32, max_val: f32, color: Color) -> Container<'static, MainAppMessage> {
    assert!(val <= max_val);

    let inner_width = (val / max_val * 1000.0).round() as u16;
    dbg!(inner_width);

    let mut inner = row![
        container(text(""))
            .style(move |_t| container::Style {
                background: Some(Background::Color(color)),
                ..Default::default()
            })
            .width(Length::FillPortion(inner_width)),
    ];

    if inner_width != 1000 {
        inner = inner.push(container(text("")).width(Length::FillPortion(1000 - inner_width)))
    }

    container(inner).style(move |_t| container::Style {
        background: Some(Background::Color(Color::BLACK)),
        ..Default::default()
    })
}

enum Lt<'a> {
    Char(&'a ButtleChar),
    Enemy(&'a ButtleEnemy),
}
impl<'a> From<&'a ButtleChar> for Lt<'a> {
    fn from(c: &'a ButtleChar) -> Self {
        Lt::Char(c)
    }
}
impl<'a> From<&'a ButtleEnemy> for Lt<'a> {
    fn from(e: &'a ButtleEnemy) -> Self {
        Lt::Enemy(e)
    }
}

impl<'a> Lt<'a> {
    fn name(&self) -> &'a str {
        match self {
            Lt::Char(c) => c.name(),
            Lt::Enemy(e) => e.info().name,
        }
    }

    fn icon_src(&self) -> String {
        match self {
            Lt::Char(c) => format!("assets/icons/{}.jpg", c.static_id()),
            Lt::Enemy(e) => format!("assets/enemy_icons/{}.jpg", e.info().id),
        }
    }

    fn lt_common(&self) -> &LtCommon {
        match self {
            Lt::Char(c) => c.lt(),
            Lt::Enemy(e) => e.lt(),
        }
    }

    fn condition(&self) -> String {
        let s = match self {
            Lt::Char(char) => match char.current_condition() {
                CharCondition::UseSkill(s) => match s.kind {
                    game_core9::skill::CharSkillProgressKind::Chanting => "詠唱中",
                    game_core9::skill::CharSkillProgressKind::Acting => "行動中",
                },
                CharCondition::Wait => "待機中",
            },
            Lt::Enemy(buttle_enemy) => match buttle_enemy.current_condition().ty {
                game_core9::buttle_enemy::EnemyConditionType::StartUp => "準備中",
                game_core9::buttle_enemy::EnemyConditionType::Recovery => "硬直中",
            },
        };

        s.to_string()
    }
}
