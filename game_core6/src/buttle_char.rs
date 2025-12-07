use crate::{
    CooldownNum,
    lt_common::LtCommon,
    passive::PassiveList,
    runtime_id::{RuntimeCharId, RuntimeSkillId},
    skill::SkillInstance,
};

pub struct ButtleChar {
    lt_common: LtCommon,
    runtime_id: RuntimeCharId,
    skills: ButtleSkills,
}

impl ButtleChar {
    pub(crate) fn new() -> Self {
        todo!()
    }

    pub(crate) fn passive_list(&self) -> &PassiveList {
        &self.lt_common.passive
    }

    pub(crate) fn get_mut_passive_list(&mut self) -> &mut PassiveList {
        &mut self.lt_common.passive
    }

    pub fn runtime_id(&self) -> RuntimeCharId {
        self.runtime_id
    }

    pub(crate) fn get_skill(&self, id: RuntimeSkillId) -> &SkillInstance {
        assert_eq!(id.char_id, self.runtime_id());
        self.skills.get(id)
    }

    pub fn skill_cooldown_heal(&self) -> CooldownNum {
        todo!()
    }
}

struct ButtleSkills {
    skills: Vec<SkillInstance>,
}

impl ButtleSkills {
    pub(crate) fn new() -> Self {
        todo!()
    }

    pub(crate) fn get(&self, skill_id: RuntimeSkillId) -> &SkillInstance {
        &self.skills[skill_id.idx as usize]
    }
}
