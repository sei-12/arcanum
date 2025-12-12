use std::collections::VecDeque;

use crate::effect::Effect;

#[derive(Debug)]
pub struct GameState {}
impl GameState {
    pub(crate) fn frame(&self, effects_buffer: &mut VecDeque<Effect>) {}
    pub(crate) fn accept(&mut self, effect: &Effect) {
        todo!()
    }
}
