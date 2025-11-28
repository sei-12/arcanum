use crate::{
    CooldownNum,
    skill::{SkillTrait, SkillWithState, StaticSkillId},
};

#[derive(Debug, Clone)]
pub struct ButtleSkills {
    inner: Vec<SkillWithState>,
}

impl ButtleSkills {
    pub fn new(skill_ids: &[StaticSkillId]) -> Self {
        let inner = skill_ids
            .iter()
            .map(|id| SkillWithState::new(*id))
            .collect();
        Self { inner }
    }

    pub fn get(&self, id: StaticSkillId) -> Result<&SkillWithState, crate::Error> {
        self.inner
            .iter()
            .find(|skill| skill.id() == id)
            .ok_or(crate::Error::NotFoundSkill { skill: id })
    }

    pub fn set_cooldown(
        &mut self,
        id: StaticSkillId,
        cooldown: CooldownNum,
    ) -> Result<(), crate::Error> {
        self.inner
            .iter_mut()
            .find(|skill| skill.id() == id)
            .ok_or(crate::Error::NotFoundSkill { skill: id })?
            .cooldown = cooldown;
        Ok(())
    }
}
