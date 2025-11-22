use game_core::{
    container_args::{CharData, ContainerArgs, EnemyData},
    game_core_actor::GameCoreActor,
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
    fn update_char(&mut self, _char: &game_core::lt::Char) {}
    fn update_enemy(&mut self, _enemy: &game_core::lt::Enemy) {}
    fn end_player_turn(&mut self) {}
    fn start_player_turn(&mut self) {}
    fn update_enemy_actions(
        &mut self,
        _actions: &std::collections::VecDeque<game_core::enemy_ai::EnemyAction>,
    ) {
    }
    fn update_enemy_mp(&mut self, _mp: f32) {}
    fn update_player_mp(&mut self, _mp: f32) {}
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
    let screen = MockGameScreen::new();
    let core = GameCoreActor::new(&arg, screen);
    assert!(core.is_ok());
}
