use std::collections::VecDeque;

use crate::{buttle_char::ButtleChar, buttle_enemy::ButtleEnemy, effect::Effect};

#[derive(Debug)]
pub struct GameState {
    chars: Vec<ButtleChar>,
    enemys: Vec<ButtleEnemy>,
}

impl GameState {
    pub(crate) fn frame(&self, effects_buffer: &mut VecDeque<Effect>) {
        self.get_chars()
            .iter()
            .for_each(|c| c.frame(self, effects_buffer));

        self.get_enemys()
            .iter()
            .for_each(|e| e.frame(self, effects_buffer));
    }

    pub(crate) fn accept(&mut self, effect: &Effect) {
        todo!()
    }

    pub fn get_chars(&self) -> &Vec<ButtleChar> {
        todo!()
    }

    pub fn get_enemys(&self) -> &Vec<ButtleEnemy> {
        todo!()
    }
}
