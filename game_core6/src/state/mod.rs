use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};

use crate::{
    LevelNum, MpNum, WinOrLoseOrNextwave,
    buttle_char::ButtleChar,
    buttle_enemy::ButtleEnemy,
    effect::Effect,
    enemy::{StaticEnemyData, StaticEnemyDataInstance},
    lt_common::LtCommon,
    runtime_id::{LtId, RuntimeCharId, RuntimeEnemyId},
};

struct EnemyData {
    level: LevelNum,
    data: StaticEnemyDataInstance,
}

type DungeonData = Arc<Vec<Vec<EnemyData>>>;

pub struct GameState {
    chars: Vec<ButtleChar>,
    current_wave_enemys: Vec<ButtleEnemy>,
    current_wave_idx: u8,
    player_mp: MpNum,
    dungeon_data: DungeonData,
}

//--------------------------------------------------//
//                                                  //
//                      ACCEPT                      //
//                                                  //
//--------------------------------------------------//
pub(crate) struct AcceptEffectResult {
    pub accepted: bool,
}

impl<'a> From<Effect> for Cow<'a, Effect> {
    fn from(val: Effect) -> Self {
        Cow::Owned(val)
    }
}

impl<'a> From<&'a Effect> for Cow<'a, Effect> {
    fn from(val: &'a Effect) -> Self {
        Cow::Borrowed(val)
    }
}

impl GameState {
    // AddPassiveの時だけPassiveInstanceをcloneする必要がある。そのためのCow。けど多分必要ないレベルで変わらん。
    pub(crate) fn accept<'a>(&mut self, effect: impl Into<Cow<'a, Effect>>) -> AcceptEffectResult {
        let effect = effect.into();

        if effect.target().is_some_and(|id| self.get_lt(id).is_dead()) {
            return AcceptEffectResult { accepted: false };
        }

        match effect.as_ref() {
            Effect::AddHate { target_id, num } => {
                self.get_mut_char(*target_id).add_hate(*num);
            }
            Effect::AddPassive {
                target_id: _,
                passive: _,
            } => {
                let Effect::AddPassive { target_id, passive } = effect.into_owned() else {
                    panic!()
                };
                self.get_lt_mut(target_id).passive.add(passive);
            }
            Effect::ConsumeMp { num } => self.player_mp = self.player_mp.saturating_sub(*num),
            Effect::ConsumeSp { target_id, num } => {
                self.get_mut_enemy(*target_id).consume_sp(*num);
            }
            Effect::Damage(dmg) => {
                self.get_lt_mut(dmg.target()).accept_damage(dmg.dmg());
            }
            Effect::HealHp { target_id, num } => {
                self.get_lt_mut(*target_id).accept_heal(*num);
            }
            Effect::HealMp { num } => {
                self.player_mp += num;
            }
            Effect::HealSkillCooldown {
                target_id,
                skill_id,
                num,
            } => {
                self.get_mut_char(*target_id)
                    .heal_skill_cooldown(*skill_id, *num);
            }
            Effect::HealSkillCooldownAll { target_id, num } => {
                self.get_mut_char(*target_id).heal_skill_cooldown_all(*num);
            }
            Effect::HealSp { target_id, num } => {
                self.get_mut_enemy(*target_id).heal_sp(*num);
            }
            Effect::SetSkillCooldown {
                target_id,
                skill_id,
                num,
            } => {
                self.get_mut_char(*target_id)
                    .set_skill_cooldown(*skill_id, *num);
            }
            Effect::UpdatePassiveState {
                target_id,
                passive_id,
                message,
            } => {
                self.get_lt_mut(*target_id)
                    .passive
                    .update_state(*passive_id, message);
            }
        }

        AcceptEffectResult { accepted: true }
    }

    pub(crate) fn go_next_wave(&mut self) {
        debug_assert!(self.current_wave_enemys_all_dead());
    }

    fn get_mut_char(&mut self, id: RuntimeCharId) -> &mut ButtleChar {
        &mut self.chars[id.idx as usize]
    }
    fn get_mut_enemy(&mut self, id: RuntimeEnemyId) -> &mut ButtleEnemy {
        assert_eq!(id.wave_idx, self.current_wave_idx);
        &mut self.current_wave_enemys[id.idx as usize]
    }
    fn get_lt_mut(&mut self, id: LtId) -> &mut LtCommon {
        match id {
            LtId::Char(c) => self.get_mut_char(c).lt_mut(),
            LtId::Enemy(e) => self.get_mut_enemy(e).lt_mut(),
        }
    }
}

//--------------------------------------------------//
//                                                  //
//                       READ                       //
//                                                  //
//--------------------------------------------------//

impl GameState {
    pub fn get_char(&self, id: RuntimeCharId) -> &ButtleChar {
        &self.chars[id.idx as usize]
    }

    pub fn get_enemy(&self, id: RuntimeEnemyId) -> &ButtleEnemy {
        assert_eq!(id.wave_idx, self.current_wave_idx);
        &self.current_wave_enemys[id.idx as usize]
    }

    pub fn get_lt(&self, id: LtId) -> &LtCommon {
        match id {
            LtId::Char(c) => self.get_char(c).lt(),
            LtId::Enemy(e) => self.get_enemy(e).lt(),
        }
    }

    pub fn enemys_with_living_check(&self) -> EnemyIterWithLivingCheck {
        EnemyIterWithLivingCheck::new()
    }

    pub fn chars_with_living_check(&self) -> CharIterWithLivingCheck {
        CharIterWithLivingCheck::new()
    }

    pub fn current_wave_is_last_wave(&self) -> bool {
        assert!(!self.dungeon_data.is_empty());
        self.dungeon_data.len() - 1 == self.current_wave_idx as usize
    }

    pub fn current_wave_enemys_all_dead(&self) -> bool {
        self.current_wave_enemys.iter().all(|e| e.lt().is_dead())
    }

    pub(crate) fn check_win_or_lose(&self) -> Result<(), WinOrLoseOrNextwave> {
        if self.chars.iter().any(|c| c.lt().is_dead()) {
            return Err(WinOrLoseOrNextwave::Lose);
        }

        if self.current_wave_enemys_all_dead() {
            if self.current_wave_is_last_wave() {
                return Err(WinOrLoseOrNextwave::Win);
            } else {
                return Err(WinOrLoseOrNextwave::Nextwave);
            }
        }

        Ok(())
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
        loop {
            let enemy = state.current_wave_enemys.get(self.idx)?;
            self.idx += 1;
            if !enemy.lt().is_dead() {
                break Some(enemy);
            }
        }
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
        loop {
            let char = state.chars.get(self.idx)?;
            self.idx += 1;
            if !char.lt().is_dead() {
                break Some(char);
            }
        }
    }
}
