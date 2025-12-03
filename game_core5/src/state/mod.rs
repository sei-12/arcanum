use crate::{UpdateStateMessage, WinOrLoseOrNextwave};


pub struct GameState {}

impl GameState {
    pub(crate) fn update(&mut self, message: &UpdateStateMessage) -> Option<WinOrLoseOrNextwave> {
        todo!()
    }
}