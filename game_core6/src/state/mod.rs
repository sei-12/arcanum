use crate::{
    WinOrLoseOrNextwave,
    effect::Effect,
    living_thing::{ButtleChar, ButtleEnemy, LtCommon},
    runtime_id::{LtId, RuntimeCharId},
};

pub(crate) struct AcceptEffectResult {
    pub accepted: bool,
}

pub struct GameState {}

impl GameState {
    pub fn get_char(&self, id: RuntimeCharId) -> &ButtleChar {
        todo!()
    }
    pub fn get_lt(&self, id: LtId) -> &LtCommon {
        todo!()
    }

    pub fn enemys_with_living_check(&self) -> EnemyIterWithLivingCheck {
        EnemyIterWithLivingCheck::new()
    }
    pub fn chars_with_living_check(&self) -> CharIterWithLivingCheck {
        CharIterWithLivingCheck::new()
    }

    pub(crate) fn accept(&mut self, effect: &Effect) -> AcceptEffectResult {
        todo!()
    }
    
    pub(crate) fn check_win_or_lose(&self) -> Result<(), WinOrLoseOrNextwave> {
        todo!()
    }
}

pub struct EnemyIterWithLivingCheck {
    idx: usize,
}

impl EnemyIterWithLivingCheck {
    pub(super) fn new() -> Self {
        Self { idx: 0 }
    }

    pub fn next_livint_enemy<'a>(&mut self, state: &'a GameState) -> Option<&'a ButtleEnemy> {
        // loop {
        //     let enemy = state.enemys.get_by_idx_from_current_wave(self.idx)?;
        //     self.idx += 1;
        //     if !enemy.lt().is_dead() {
        //         break Some(enemy);
        //     }
        // }
        todo!()
    }
}

pub struct CharIterWithLivingCheck {
    idx: usize,
}
impl CharIterWithLivingCheck {
    pub(super) fn new() -> Self {
        Self { idx: 0 }
    }

    pub fn next_livint_char<'a>(&mut self, state: &'a GameState) -> Option<&'a ButtleChar> {
        // loop {
        //     let char = state.chars.get_by_idx(self.idx)?;
        //     self.idx += 1;
        //     if !char.lt().is_dead() {
        //         break Some(char);
        //     }
        // }
        todo!()
    }
}
