use crate::{
    CooldownNum, HateNum, LevelNum, NUM_MAX_LEARN_SKILLS, SKILL_COOLDOWN_HEAL_BASE, StaticCharId,
    lt_common::LtCommon,
    passive::PassiveInstance,
    potential::Potential,
    runtime_id::{RuntimeCharId, RuntimeSkillId},
    skill::{SkillInstance, SkillUpdateMessage},
};

#[derive(Debug, Clone)]
pub struct ButtleChar {
    lt_common: LtCommon,
    runtime_id: RuntimeCharId,
    skills: Vec<ButtleSkill>,
    static_data: StaticCharData,
    hate: HateNum,
}

impl ButtleChar {
    pub(crate) fn new(
        runtime_char_id: RuntimeCharId,
        level: LevelNum,
        static_data: StaticCharData,
        skills: Vec<SkillInstance>,
    ) -> Result<Self, crate::Error> {
        assert!(level > 0);

        if skills.is_empty() || skills.len() > NUM_MAX_LEARN_SKILLS {
            return Err(crate::Error::InvalidNumLearnSkills(skills.len()));
        }

        let skills = skills
            .into_iter()
            .enumerate()
            .map(|(i, skill)| ButtleSkill {
                id: RuntimeSkillId {
                    char_id: runtime_char_id,
                    idx: i as u8,
                },
                cooldown: 0,
                instance: skill,
            })
            .collect();

        Ok(Self {
            lt_common: LtCommon::new(static_data.potential.clone(), level),
            runtime_id: runtime_char_id,
            skills,
            static_data,
            hate: 0,
        })
    }

    pub fn lt(&self) -> &LtCommon {
        &self.lt_common
    }
    pub fn lt_mut(&mut self) -> &mut LtCommon {
        &mut self.lt_common
    }

    pub fn static_data(&self) -> &StaticCharData {
        &self.static_data
    }

    pub fn runtime_id(&self) -> RuntimeCharId {
        self.runtime_id
    }

    pub(crate) fn get_skill(&self, id: RuntimeSkillId) -> &SkillInstance {
        assert_eq!(id.char_id, self.runtime_id());
        &self.skills[id.idx as usize].instance
    }

    pub fn skill_cooldown_heal(&self) -> CooldownNum {
        SKILL_COOLDOWN_HEAL_BASE + (self.lt().agi() * 5.0) as u32
    }

    pub(crate) fn set_skill_cooldown(&mut self, id: RuntimeSkillId, num: CooldownNum) {
        self.skills[id.idx as usize].cooldown = num;
    }
    pub(crate) fn heal_skill_cooldown(&mut self, id: RuntimeSkillId, num: CooldownNum) {
        assert_eq!(id.char_id, self.runtime_id());
        self.skills[id.idx as usize].heal_cooldown(num);
    }

    pub(crate) fn heal_skill_cooldown_all(&mut self, num: CooldownNum) {
        self.skills.iter_mut().for_each(|state| {
            state.heal_cooldown(num);
        });
    }

    pub(crate) fn add_hate(&mut self, num: HateNum) {
        self.hate = self.hate.saturating_add(num);
    }

    pub(crate) fn update_skill_state(&mut self, id: RuntimeSkillId, msg: &SkillUpdateMessage) {
        self.skills[id.idx as usize].instance.update(msg);
    }

    pub fn skills(&self) -> &Vec<ButtleSkill> {
        &self.skills
    }
}


#[derive(Debug, Clone)]
pub struct ButtleSkill {
    pub id: RuntimeSkillId,
    cooldown: CooldownNum,
    instance: SkillInstance,
}

impl ButtleSkill {
    fn heal_cooldown(&mut self, num: CooldownNum) {
        self.cooldown = self.cooldown.saturating_sub(num);
    }
}

#[derive(Debug, Clone)]
pub struct StaticCharData {
    pub id: StaticCharId,
    pub potential: Potential,
    pub passives: fn() -> Vec<PassiveInstance>,
}
