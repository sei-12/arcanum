use std::collections::VecDeque;

use crate::{
    buttle_char::ButtleChar,
    buttle_enemy::ButtleEnemy,
    core_actor::CtxContainer,
    effect::Effect,
    lt_common::LtCommon,
    runtime_id::{LtId, RuntimeCharId},
};

#[derive(Debug)]
pub struct GameState {
    chars: Vec<ButtleChar>,
    enemys: Vec<ButtleEnemy>,
}

impl GameState {
    pub(crate) fn new() -> Result<Self, crate::Error> {
        todo!()
    }

    pub(crate) fn frame(&self, ctx: &mut CtxContainer) {
        self.get_chars().iter().for_each(|c| c.frame(self, ctx));

        self.get_enemys().iter().for_each(|e| e.frame(self, ctx));
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

    pub fn get_char(&self, id: RuntimeCharId) -> &ButtleChar {
        todo!()
    }

    pub fn get_enemy(&self, id: RuntimeCharId) -> &ButtleEnemy {
        todo!()
    }

    pub fn try_get_char(&self, id: RuntimeCharId) -> Result<&ButtleChar, crate::Error> {
        todo!()
    }

    pub fn try_get_enemy(&self, id: RuntimeCharId) -> Result<&ButtleEnemy, crate::Error> {
        todo!()
    }
    pub fn get_lt(&self, lt_id: LtId) -> &LtCommon {
        todo!()
    }
}
