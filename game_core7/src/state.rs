use crate::{
    NUM_MAX_CHAR_IN_TEAM, NUM_MAX_ENEMYS_IN_WAVE,
    buttle_char::{ButtleChar, ButtleCharArgs},
    buttle_enemy::{ButtleEnemy, ButtleEnemyArgs},
    core_actor::CtxContainer,
    effect::Effect,
    lt_common::LtCommon,
    runtime_id::{LtId, RuntimeCharId, RuntimeEnemyId},
};

#[derive(Debug)]
pub struct GameState {
    chars: Vec<ButtleChar>,
    enemys: Vec<ButtleEnemy>,
}

pub struct GameStateArgs {
    pub chars: Vec<ButtleCharArgs>,
    pub enemys: Vec<ButtleEnemyArgs>,
}

impl GameState {
    pub(crate) fn new(args: GameStateArgs) -> Result<Self, crate::Error> {
        let char_datas = args.chars;
        let enemy_datas = args.enemys;

        let mut chars = Vec::<ButtleChar>::new();

        if char_datas.is_empty() || char_datas.len() > NUM_MAX_CHAR_IN_TEAM as usize {
            return Err(crate::Error::InvalidNumTeamMembers(char_datas.len()));
        }

        for (i, char_data) in char_datas.into_iter().enumerate() {
            let runtime_id = RuntimeCharId { idx: i as u8 };
            chars.push(ButtleChar::new(runtime_id, char_data)?);
        }

        let mut enemys = Vec::<ButtleEnemy>::new();
        if enemy_datas.is_empty() || enemy_datas.len() > NUM_MAX_ENEMYS_IN_WAVE {
            return Err(crate::Error::InvalidNumEnemys(enemy_datas.len()));
        }

        for (i, enemy_data) in enemy_datas.into_iter().enumerate() {
            let runtime_id = RuntimeEnemyId { idx: i as u8 };
            enemys.push(ButtleEnemy::new(runtime_id, enemy_data)?);
        }

        Ok(Self { chars, enemys })
    }

    pub(crate) fn tick(&mut self) {
        self.chars.iter_mut().for_each(|c| c.tick());
        self.enemys.iter_mut().for_each(|e| e.tick());
    }

    pub(crate) fn frame(&self, ctx: &mut CtxContainer) {
        self.get_chars().iter().for_each(|c| c.frame(self, ctx));

        self.get_enemys().iter().for_each(|e| e.frame(self, ctx));
    }

    pub(crate) fn accept(&mut self, effect: &Effect) {
        match effect {
            Effect::HealHp { target_id, num } => {
                self.get_lt_mut(*target_id).accept_heal(*num);
            }
            Effect::AddSkillCooldown {
                target_id,
                skill_id,
                num,
            } => {
                self.get_char_mut(*target_id)
                    .add_skill_cooldown(*skill_id, *num);
            }
            Effect::HealSkillCooldown {
                target_id,
                skill_id,
                num,
            } => {
                self.get_char_mut(*target_id)
                    .heal_skill_cooldown(*skill_id, *num);
            }
            Effect::HealSkillCooldownAll { target_id, num } => {
                self.get_char_mut(*target_id).heal_skill_cooldown_all(*num);
            }
            Effect::HealHate { target_id, num } => {
                self.get_char_mut(*target_id).heal_hate(*num);
            }
            Effect::AddHate { target_id, num } => {
                self.get_char_mut(*target_id).add_hate(*num);
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
            Effect::Damage(damage) => {
                self.get_lt_mut(damage.target()).accept_damage(damage.dmg());
            }
            Effect::UseSkill {
                user_id,
                skill_id,
                state,
            } => {
                self.get_char_mut(*user_id)
                    .spawn_skill_action(*skill_id, state.clone());
            }
            Effect::AddPassive { target_id, passive } => {
                self.get_lt_mut(*target_id).passive.add(passive.clone());
            }
            Effect::UpdateSkillState {
                target_id,
                skill_id,
                msg,
            } => {
                self.get_char_mut(*target_id)
                    .update_skill_state(*skill_id, msg);
            }
            Effect::ConsumeMp { target_id, num } => {
                self.get_lt_mut(*target_id).accept_consume_mp(*num);
            }
            Effect::HealMp { target_id, num } => {
                self.get_lt_mut(*target_id).accept_heal_mp(*num);
            }
        }
    }

    fn get_char_mut(&mut self, id: RuntimeCharId) -> &mut ButtleChar {
        &mut self.chars[id.idx as usize]
    }

    fn get_enemy_mut(&mut self, id: RuntimeEnemyId) -> &mut ButtleEnemy {
        &mut self.enemys[id.idx as usize]
    }

    fn get_lt_mut(&mut self, id: LtId) -> &mut LtCommon {
        match id {
            LtId::Char(char_id) => self.get_char_mut(char_id).lt_mut(),
            LtId::Enemy(enemy_id) => self.get_enemy_mut(enemy_id).lt_mut(),
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

    pub fn get_enemys(&self) -> &Vec<ButtleEnemy> {
        &self.enemys
    }

    pub fn get_char(&self, id: RuntimeCharId) -> &ButtleChar {
        &self.chars[id.idx as usize]
    }

    pub fn get_enemy(&self, id: RuntimeEnemyId) -> &ButtleEnemy {
        &self.enemys[id.idx as usize]
    }

    pub fn try_get_char(&self, id: RuntimeCharId) -> Result<&ButtleChar, crate::Error> {
        self.chars
            .get(id.idx as usize)
            .ok_or(crate::Error::NotFoundChar(id))
    }

    pub fn try_get_enemy(&self, id: RuntimeCharId) -> Result<&ButtleEnemy, crate::Error> {
        self.enemys
            .get(id.idx as usize)
            .ok_or(crate::Error::NotFoundEnemy(id))
    }

    pub fn get_lt(&self, lt_id: LtId) -> &LtCommon {
        match lt_id {
            LtId::Char(id) => self.get_char(id).lt(),
            LtId::Enemy(id) => self.get_enemy(id).lt(),
        }
    }
}
