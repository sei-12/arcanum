use crate::{
    GameResult,
    buttle_enemy::ButtleEnemy,
    event::Event,
    state::chars::{ButtleChars, RuntimeCharId},
};

pub mod chars;

#[derive(Debug, Clone, Copy)]
pub enum LtId {
    Enemy,
    Char(RuntimeCharId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Player,
    Enemy,
}

pub struct GameState {
    chars: ButtleChars,
    enemy: ButtleEnemy,
}

impl GameState {
    pub fn accept_event(&mut self, event: Event) {
        todo!()
    }

    pub fn chars(&self) -> &ButtleChars {
        &self.chars
    }

    pub fn enemy(&self) -> &ButtleEnemy {
        &self.enemy
    }

    pub fn check_game_end(&self) -> Option<GameResult> {
        if self.chars.chars().iter().any(|char| char.lt().is_dead()) {
            return Some(GameResult::Lose);
        }
        if self.enemy.lt().is_dead() {
            return Some(GameResult::Win);
        }
        None
    }
}
