use std::vec;

use crate::{
    container::Container,
    container_args::{CharData, ContainerArgs, EnemyData},
    error::GameError,
    game_state::GameState,
    screen_actor::ScreenActorSender,
};

#[derive(Debug)]
struct MockGameScreen {}
impl MockGameScreen {
    fn new() -> Box<Self> {
        Box::new(Self {})
    }
}
impl ScreenActorSender for MockGameScreen {
    fn update_char(&mut self, _char: &crate::lt::Char) {}
    fn update_enemy(&mut self, _enemy: &crate::lt::Enemy) {}
    fn end_player_turn(&mut self) {}
    fn start_player_turn(&mut self) {}
    fn update_enemy_actions(
        &mut self,
        _actions: &std::collections::VecDeque<crate::enemy_ai::EnemyAction>,
    ) {
    }
    fn update_enemy_mp(&mut self, _mp: crate::Num) {}
    fn update_player_mp(&mut self, _mp: crate::Num) {}
    fn log(&mut self, _msg: std::borrow::Cow<'_, str>) {}
    fn lose(&mut self) {}
    fn win(&mut self) {}
    fn initialize(&mut self, _state: &GameState) {}
}

#[test]
fn test1() {
    let arg = ContainerArgs {
        chars: vec![
            CharData {
                level: 1.0,
                own_skill_ids: vec![0],
                static_char_id: 0,
            },
            CharData {
                level: 1.0,
                own_skill_ids: vec![0],
                static_char_id: 1,
            },
        ],
        enemy: EnemyData {
            level: 1.0,
            static_enemy_id: 0,
        },
    };

    let game_screen = MockGameScreen::new();
    let container = Container::new(&arg, game_screen).unwrap();

    assert_eq!(container.get_char(0).unwrap().static_data.name, "エレナ");
    assert_eq!(container.get_char(1).unwrap().static_data.name, "ゆら");
}

#[test]
fn confrict_char() {
    let arg = ContainerArgs {
        chars: vec![
            CharData {
                level: 1.0,
                own_skill_ids: vec![0],
                static_char_id: 0,
            },
            CharData {
                level: 1.0,
                own_skill_ids: vec![0],
                static_char_id: 0,
            },
        ],
        enemy: EnemyData {
            level: 1.0,
            static_enemy_id: 0,
        },
    };

    let game_screen = MockGameScreen::new();
    let err = Container::new(&arg, game_screen).unwrap_err();

    assert_eq!(err, GameError::ConfrictChar)
}

// 現状は存在しないキャラのIDだが、いつか存在するIDになる可能性があるのでその時は書き直してもろて
#[test]
fn invalid_char_id() {
    let arg = ContainerArgs {
        chars: vec![CharData {
            level: 1.0,
            own_skill_ids: vec![0],
            static_char_id: 10000,
        }],
        enemy: EnemyData {
            level: 1.0,
            static_enemy_id: 0,
        },
    };

    let game_screen = MockGameScreen::new();
    let err = Container::new(&arg, game_screen).unwrap_err();

    assert_eq!(err, GameError::InvalidCharId)
}

#[test]
fn invalid_enemy_id() {
    let arg = ContainerArgs {
        chars: vec![CharData {
            level: 1.0,
            own_skill_ids: vec![0],
            static_char_id: 0,
        }],
        enemy: EnemyData {
            level: 1.0,
            static_enemy_id: 10000,
        },
    };

    let game_screen = MockGameScreen::new();
    let err = Container::new(&arg, game_screen).unwrap_err();

    assert_eq!(err, GameError::InvalidEnemyId)
}

#[test]
fn invalid_skill_id() {
    let arg = ContainerArgs {
        chars: vec![CharData {
            level: 1.0,
            own_skill_ids: vec![100000],
            static_char_id: 0,
        }],
        enemy: EnemyData {
            level: 1.0,
            static_enemy_id: 0,
        },
    };

    let game_screen = MockGameScreen::new();
    let err = Container::new(&arg, game_screen).unwrap_err();

    assert_eq!(err, GameError::InvalidSkillId)
}
