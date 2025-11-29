use std::sync::mpsc;

use game_core3::{GameResult, state::GameState};

pub struct EventReceiver {
    recv: mpsc::Receiver<game_core3::event::Event>,
    state: GameState,
    log: Vec<String>,
    result: Option<GameResult>,
}

impl EventReceiver {
    pub fn new(recv: mpsc::Receiver<game_core3::event::Event>, state: GameState) -> Self {
        Self {
            recv,
            state,
            log: Vec::new(),
            result: None,
        }
    }

    pub fn update(&mut self) {
        while let Ok(event) = self.recv.try_recv() {
            match &event {
                game_core3::event::Event::Log(log) => {
                    self.log.push(log.clone());
                }
                game_core3::event::Event::GameEnd(result) => {
                    self.result = Some(*result);
                }
                _ => {}
            };

            self.state.accept_event(event);
        }
    }

    pub fn log(&self) -> &Vec<String> {
        &self.log
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn result(&self) -> &Option<GameResult> {
        &self.result
    }
}
