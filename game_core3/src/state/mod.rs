use crate::{
    event::Event,
    state::chars::{ButtleChars, RuntimeCharId},
};

pub mod chars;

pub enum LtId {
    Enemy,
    Char(RuntimeCharId),
}

pub struct GameState {
    chars: ButtleChars,
}

impl GameState {
    pub fn accept_event(&mut self, event: Event) {
        todo!()
    }
}
