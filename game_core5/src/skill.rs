use crate::{
    CooldownNum, HateNum, MpNum, NUM_MAX_LEARN_SKILLS, StaticSkillId, WinOrLoseOrNextwave,
    effector::Effecter,
    state::{RuntimeCharId, RuntimeEnemyId},
};

pub struct SkillCost {
    pub need_mp: MpNum,
    pub hate: HateNum,
    pub cooldown: CooldownNum,
}

/// (user_id, target_enemy_id, effector)
pub type SkillFn = fn(
    RuntimeCharId,
    Option<RuntimeEnemyId>,
    &mut Effecter,
) -> Result<SkillCost, WinOrLoseOrNextwave>;

#[derive(Debug)]
pub struct StaticSkillData {
    pub id: StaticSkillId,
    pub call: SkillFn,
}

#[derive(Debug, Clone)]
pub struct ButtleSkills {
    inner: Vec<ButtleSkill>,
}

impl ButtleSkills {
    pub(crate) fn new(
        owner: RuntimeCharId,
        skills: &[&'static StaticSkillData],
    ) -> Result<Self, crate::Error> {
        if skills.is_empty() || skills.len() > NUM_MAX_LEARN_SKILLS {
            return Err(crate::Error::InvalidNumLearnSkills(skills.len()));
        }

        let inner = skills
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let runtime_id = RuntimeSkillId(owner, i as u8);
                ButtleSkill {
                    cooldown: 0,
                    runtime_id,
                    static_data: s,
                }
            })
            .collect();

        Ok(Self { inner })
    }

    pub(crate) fn get(&self, id: RuntimeSkillId) -> &ButtleSkill {
        &self.inner[id.1 as usize]
    }

    pub(crate) fn set_cooldown(&mut self, id: RuntimeSkillId, cooldown: CooldownNum) {
        self.inner
            .iter_mut()
            .find(|skill| skill.runtime_id == id)
            .unwrap()
            .cooldown = cooldown;
    }

    pub(crate) fn heal_skill_cooldown(&mut self, target_id: RuntimeSkillId, heal_num: CooldownNum) {
        let target_item = self
            .inner
            .iter_mut()
            .find(|skill| skill.runtime_id == target_id)
            .unwrap();

        if target_item.cooldown > heal_num {
            target_item.cooldown -= heal_num;
        } else {
            target_item.cooldown = 0;
        };
    }

    pub(crate) fn heal_skill_cooldown_all(&mut self, heal_num: CooldownNum) {
        for item in self.inner.iter_mut() {
            if item.cooldown > heal_num {
                item.cooldown -= heal_num;
            } else {
                item.cooldown = 0;
            };
        }
    }
}

#[derive(Debug, Clone)]
pub struct ButtleSkill {
    runtime_id: RuntimeSkillId,
    cooldown: CooldownNum,
    static_data: &'static StaticSkillData,
}

impl ButtleSkill {
    pub fn runtime_id(&self) -> RuntimeSkillId {
        self.runtime_id
    }
    pub fn static_data(&self) -> &'static StaticSkillData {
        self.static_data
    }

    pub fn cooldown(&self) -> CooldownNum {
        self.cooldown
    }

    pub(crate) fn get_skill_fn(&self) -> SkillFn {
        self.static_data.call
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeSkillId(RuntimeCharId, u8);
