use crate::{event::EventsQuePusher, lt_common::LtCommon, state::GameState};

pub struct ButtleEnemy {}

impl ButtleEnemy {
    pub fn lt(&self) -> &LtCommon {
        todo!()
    }
    pub fn lt_mut(&mut self) -> &mut LtCommon {
        todo!()
    }

    pub fn play_action(&self, state: &GameState, events: &mut impl EventsQuePusher) {}
}
