use std::{collections::VecDeque, sync::mpsc};

use game_core::{game_state::GameResult, screen_actor::ScreenActorSender};

#[derive(Debug)]
struct Sender {
    sender: mpsc::Sender<Msg>,
}

impl ScreenActorSender for Sender {
    fn end_player_turn(&mut self) {}
    fn initialize(&mut self, _state: &game_core::game_state::GameState) {}

    fn start_player_turn(&mut self) {}
    fn update_char(&mut self, _char: &game_core::lt::Char) {}
    fn update_enemy(&mut self, _enemy: &game_core::lt::Enemy) {}
    fn update_enemy_actions(
        &mut self,
        _actions: &std::collections::VecDeque<game_core::enemy_ai::EnemyAction>,
    ) {
    }
    fn update_enemy_mp(&mut self, _mp: f32) {}
    fn update_player_mp(&mut self, _mp: f32) {}

    fn log(&mut self, msg: std::borrow::Cow<'_, str>) {
        // msg
        self.sender
            .send(Msg::Log(msg.into_owned()))
            .expect("failed to send message");
    }
    fn lose(&mut self) {
        self.sender
            .send(Msg::Result(GameResult::Lose))
            .expect("failed to send message");
    }
    fn win(&mut self) {
        self.sender
            .send(Msg::Result(GameResult::Win))
            .expect("failed to send message");
    }
}

enum Msg {
    Log(String),
    Result(GameResult),
}

#[derive(Debug)]
pub struct ScreenActor {
    recv: mpsc::Receiver<Msg>,
    logs: VecDeque<String>,
    result: GameResult,
}

impl ScreenActor {
    pub fn update(&mut self) {
        while let Ok(msg) = self.recv.try_recv() {
            match msg {
                Msg::Log(log) => self.logs.push_back(log),
                Msg::Result(result) => self.result = result,
            }
        }
    }

    pub fn get_result(&self) -> GameResult {
        self.result
    }

    pub fn get_log(&self) -> &VecDeque<String> {
        &self.logs
    }
}

pub fn get_screen_actor() -> (ScreenActor, Box<dyn ScreenActorSender>) {
    let (sender, recv) = mpsc::channel();
    let screen_actor = ScreenActor {
        logs: VecDeque::default(),
        recv,
        result: GameResult::None,
    };

    let sender = Box::new(Sender { sender });

    (screen_actor, sender)
}
