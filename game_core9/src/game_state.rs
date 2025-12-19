use crate::{
    MAX_CHARACTERS,
    buttle_char::{ButtleChar, ButtleCharArgs},
    buttle_enemy::{ButtleEnemy, ButtleEnemyArgs},
    buttle_skill::ButtleSkill,
    core_actor::EffectsBuffer,
    effect::Effect,
    lt_common::LtCommon,
    runtime_id::{LtId, RuntimeCharId, RuntimeSkillId},
};

pub struct GameStateArgs {
    pub chars: Vec<ButtleCharArgs>,
    pub enemy: ButtleEnemyArgs,
}

#[derive(Debug)]
pub struct GameState {
    chars: Vec<ButtleChar>,
    enemy: ButtleEnemy,
}

//--------------------------------------------------//
//                                                  //
//                     PRIVATE                      //
//                                                  //
//--------------------------------------------------//
impl GameState {
    pub(crate) fn new(args: GameStateArgs) -> Result<Self, crate::Error> {
        let mut chars = Vec::with_capacity(args.chars.len());

        if args.chars.is_empty() || args.chars.len() > MAX_CHARACTERS {
            return Err(crate::Error::InvalidArgument(
                "Number of characters is invalid".to_string(),
            ));
        }

        for (i, c) in args.chars.into_iter().enumerate() {
            let runtime_id = RuntimeCharId { idx: i as u8 };
            chars.push(ButtleChar::new(runtime_id, c)?);
        }

        let enemy = ButtleEnemy::new(args.enemy)?;

        Ok(Self { chars, enemy })
    }

    pub(crate) fn tick(&self, effects_buffer: &mut EffectsBuffer) {
        for char in self.chars.iter() {
            char.tick(self, effects_buffer);
        }
        self.enemy.tick(self, effects_buffer);
    }

    pub(crate) fn accept_effect(&mut self, effect: &Effect) {
        match effect {
            Effect::Damage(damage) => {
                self.get_lt_mut(damage.target()).accept_damage(damage.dmg());
            }
            Effect::HealHp { target_id, num } => {
                self.get_lt_mut(*target_id).accept_heal(*num);
            }
            Effect::ConsumeMp { target_id, num } => {
                self.get_lt_mut(*target_id).accept_consume_mp(*num);
            }
            Effect::HealMp { target_id, num } => {
                self.get_lt_mut(*target_id).accept_heal_mp(*num);
            }
            Effect::UpdatePassiveState {
                target_id,
                passive_id,
                message,
            } => {
                self.get_lt_mut(*target_id)
                    .passive
                    .update(*passive_id, message);
            }
            Effect::AddPassive { target_id, passive } => {
                self.get_lt_mut(*target_id).passive.add(passive.clone());
            }
            Effect::AddSkillCooldown { skill_id, num } => {
                self.get_char_mut(skill_id.char_id)
                    .get_skill_mut(*skill_id)
                    .add_cooldown(*num);
            }
            Effect::HealSkillCooldownAll { target_id, num } => {
                self.get_char_mut(*target_id).heal_skill_cooldown_all(*num);
            }
            Effect::AddHate { target_id, num } => {
                self.get_char_mut(*target_id).add_hate(*num);
            }
            Effect::UpdateSkillState { skill_id, msg } => {
                self.get_skill_mut(*skill_id).skill_box_mut().update(msg);
            }
            Effect::UseSkill {
                skill_id,
            } => {
                assert!(
                    self.get_skill(*skill_id).useable(self),
                    "チェック済みである必要がある"
                );
                self.get_char_mut(skill_id.char_id).use_skill(*skill_id);
            }
            Effect::EndSkill { skill_id } => {
                self.get_char_mut(skill_id.char_id).end_skill(*skill_id);
            }
            Effect::EnemySkillRunnerIncrementFrame => {
                self.enemy.skill_runner_increment_frame();
            }
        }
    }

    fn get_lt_mut(&mut self, id: LtId) -> &mut LtCommon {
        match id {
            LtId::Enemy => self.enemy.lt_mut(),
            LtId::Char(runtime_id) => self.get_char_mut(runtime_id).lt_mut(),
        }
    }

    fn get_char_mut(&mut self, id: RuntimeCharId) -> &mut ButtleChar {
        self.chars.get_mut(id.idx as usize).unwrap()
    }
}

//--------------------------------------------------//
//                                                  //
//                      PUBLIC                      //
//                                                  //
//--------------------------------------------------//
impl GameState {
    pub fn get_char(&self, id: RuntimeCharId) -> &ButtleChar {
        self.chars.get(id.idx as usize).unwrap()
    }

    pub fn get_enemy(&self) -> &ButtleEnemy {
        &self.enemy
    }

    pub fn get_skill(&self, id: RuntimeSkillId) -> &ButtleSkill {
        self.get_char(id.char_id).get_skill(id)
    }

    pub fn get_skill_mut(&mut self, id: RuntimeSkillId) -> &mut ButtleSkill {
        self.get_char_mut(id.char_id).get_skill_mut(id)
    }

    pub fn get_lt(&self, id: LtId) -> &LtCommon {
        match id {
            LtId::Enemy => self.enemy.lt(),
            LtId::Char(runtime_id) => self.get_char(runtime_id).lt(),
        }
    }

    pub(crate) fn check_win_or_lose(&self) -> Option<WinOrLose> {
        if self.chars.iter().any(|c| c.lt().is_dead()) {
            return Some(WinOrLose::Lose);
        }

        if self.enemy.lt().is_dead() {
            return Some(WinOrLose::Win);
        }

        None
    }

    pub fn get_chars(&self) -> &Vec<ButtleChar> {
        &self.chars
    }

    pub fn get_highest_hate_char(&self) -> &ButtleChar {
        assert!(!self.chars.is_empty());

        self.chars
            .iter()
            .rev()
            .max_by(|a, b| a.hate().total_cmp(&b.hate()))
            .unwrap()
    }

    pub fn get_chars_sorted_by_hate(&self) -> Vec<&ButtleChar> {
        let mut tmp = self.chars.iter().collect::<Vec<_>>();
        tmp.sort_by(|a, b| b.hate().total_cmp(&a.hate()));

        debug_assert!({ tmp.first().unwrap().hate() >= tmp.last().unwrap().hate() });

        tmp
    }
}

pub(crate) enum WinOrLose {
    Win,
    Lose,
}
