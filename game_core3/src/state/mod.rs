use crate::{
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

#[derive(Debug, Clone, Copy, PartialEq,Eq)]
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
}
