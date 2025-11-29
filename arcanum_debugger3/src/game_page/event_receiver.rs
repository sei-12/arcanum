use std::sync::mpsc;

use game_core3::{
    GameResult,
    event::Event,
    state::{GameState, LtId},
};

use crate::game_page::event_to_log::event_to_log;

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
            if let Some(msg) = event_to_log(&event, &self.state) {
                self.log.push(msg);
            }

            if let Event::GameEnd(result) = &event {
                self.result = Some(*result);
            }

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
