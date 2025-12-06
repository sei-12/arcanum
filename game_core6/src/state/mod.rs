use crate::{living_thing::ButtleChar, runtime_id::RuntimeCharId};

pub struct GameState {}

impl GameState {
    pub fn get_char(&self, id: RuntimeCharId) -> &ButtleChar {
        todo!()
    }
}
