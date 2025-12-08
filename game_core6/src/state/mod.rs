use std::{borrow::Cow, collections::HashSet, sync::Arc};

use crate::{
    LevelNum, MpNum, NUM_MAX_CHAR_IN_TEAM, NUM_MAX_ENEMYS_IN_WAVE, NUM_MAX_WAVES,
    WinOrLoseOrNextwave,
    buttle_char::{ButtleChar, StaticCharData},
    buttle_enemy::ButtleEnemy,
    effect::Effect,
    enemy::StaticEnemyDataInstance,
    lt_common::LtCommon,
    runtime_id::{LtId, RuntimeCharId, RuntimeEnemyId},
    skill::SkillInstance,
};

#[derive(Debug)]
pub struct EnemyData {
    pub level: LevelNum,
    pub data: StaticEnemyDataInstance,
}

pub type DungeonData = Arc<Vec<Vec<EnemyData>>>;

#[derive(Debug, Clone)]
struct WrapDungeonData(DungeonData);
impl WrapDungeonData {
    fn new(data: DungeonData) -> Result<Self, crate::Error> {
        if data.is_empty() || data.len() > NUM_MAX_WAVES {
            return Err(crate::Error::InvalidNumWaves(data.len()));
        };

        if data
            .iter()
            .any(|wave| wave.is_empty() || wave.len() > NUM_MAX_ENEMYS_IN_WAVE)
        {
            return Err(crate::Error::InvalidNumEnemysInWave(data));
        }
        Ok(Self(data))
    }

    fn waves(&self) -> usize {
        self.0.len()
    }

    fn make_wave(&self, wave_idx: usize) -> Vec<ButtleEnemy> {
        self.0
            .get(wave_idx)
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let runtime_id = RuntimeEnemyId {
                    idx: i as u8,
                    wave_idx: wave_idx as u8,
                };
                ButtleEnemy::new(runtime_id, e.level, e.data.clone())
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    chars: Vec<ButtleChar>,
    current_wave_enemys: Vec<ButtleEnemy>,
    current_wave_idx: u8,
    player_mp: MpNum,
    dungeon_data: WrapDungeonData,
}

pub struct CharData {
    pub level: LevelNum,
    pub data: StaticCharData,
    pub skills: Vec<SkillInstance>,
}

impl GameState {
    pub(crate) fn new(
        char_datas: Vec<CharData>,
        dungeon_data: DungeonData,
    ) -> Result<Self, crate::Error> {
        let mut chars = Vec::new();
        if char_datas.is_empty() || char_datas.len() > NUM_MAX_CHAR_IN_TEAM as usize {
            return Err(crate::Error::InvalidNumTeamMembers {
                got_num_members: char_datas.len(),
            });
        }

        for (i, char_data) in char_datas.into_iter().enumerate() {
            let runtime_id = RuntimeCharId { idx: i as u8 };
            chars.push(ButtleChar::new(
                runtime_id,
                char_data.level,
                char_data.data,
                char_data.skills,
            )?);
        }

        let dungeon_data = WrapDungeonData::new(dungeon_data)?;
        let current_wave_enemys = dungeon_data.make_wave(0);

        Ok(Self {
            chars,
            current_wave_enemys,
            current_wave_idx: 0,
            player_mp: 0,
            dungeon_data,
        })
    }
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
            Effect::UpdateSkillState {
                target_id,
                skill_id,
                msg,
            } => {
                self.get_mut_char(*target_id)
                    .update_skill_state(*skill_id, msg);
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
    pub fn get_chars(&self) -> &Vec<ButtleChar> {
        &self.chars
    }

    pub fn get_current_wave_enemys(&self) -> &Vec<ButtleEnemy> {
        &self.current_wave_enemys
    }

    pub fn get_char(&self, id: RuntimeCharId) -> &ButtleChar {
        &self.chars[id.idx as usize]
    }

    pub fn get_current_wave_enemy(&self, id: RuntimeEnemyId) -> &ButtleEnemy {
        assert_eq!(id.wave_idx, self.current_wave_idx);
        &self.current_wave_enemys[id.idx as usize]
    }

    pub fn get_lt(&self, id: LtId) -> &LtCommon {
        match id {
            LtId::Char(c) => self.get_char(c).lt(),
            LtId::Enemy(e) => self.get_current_wave_enemy(e).lt(),
        }
    }

    pub fn enemys_with_living_check(&self) -> EnemyIterWithLivingCheck {
        EnemyIterWithLivingCheck::new()
    }

    pub fn chars_with_living_check(&self) -> CharIterWithLivingCheck {
        CharIterWithLivingCheck::new()
    }

    pub fn current_wave_is_last_wave(&self) -> bool {
        self.dungeon_data.waves() - 1 == self.current_wave_idx as usize
    }

    pub fn current_wave_enemys_all_dead(&self) -> bool {
        self.current_wave_enemys.iter().all(|e| e.lt().is_dead())
    }

    /// 現在のウェーブに存在する敵のうち、ターゲット優先度の高い順で
    /// 敵を走査するイテレータを返します。
    ///
    /// ### 戻り値に関する保証
    ///
    /// この関数が返すイテレータは、次の 3 点を満たすことが保証されます。
    ///
    /// 1. イテレータは空ではありません
    /// 2. 全ての要素は生存状態の敵です
    /// 3. 同じ `RuntimeEnemyId` を持つ敵が複数含まれることはありません
    pub fn get_enemys_highest_target_priority(
        &self,
        target: Option<RuntimeEnemyId>,
    ) -> impl Iterator<Item = &ButtleEnemy> {
        assert!(!self.current_wave_enemys_all_dead());

        let iter = target
            .map(|target_id| self.get_current_wave_enemy(target_id))
            .into_iter()
            .chain(
                self.current_wave_enemys
                    .iter()
                    .filter(move |e| Some(e.runtime_id()) != target),
            )
            .filter(|e| !e.lt().is_dead());

        debug_assert!({ iter.clone().count() > 0 });
        debug_assert!({ iter.clone().all(|e| !e.lt().is_dead()) });
        // 同じIDを持つ敵がイテレータ内に2つ以上存在しないことを確認
        debug_assert!({
            let iter_count = iter.clone().count();
            let hash_set_len = iter
                .clone()
                .map(|e| e.runtime_id())
                .collect::<HashSet<_>>()
                .len();

            iter_count == hash_set_len
        });

        iter
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

    pub fn player_mp(&self) -> MpNum {
        self.player_mp
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
