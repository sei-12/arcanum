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

    fn get_mut(&mut self, id: StaticSkillId) -> Result<&mut SkillWithState, crate::Error> {
        self.inner
            .iter_mut()
            .find(|skill| skill.id() == id)
            .ok_or(crate::Error::NotFoundSkill { skill: id })
    }

    pub fn get(&self, id: StaticSkillId) -> Result<&SkillWithState, crate::Error> {
        self.inner
            .iter()
            .find(|skill| skill.id() == id)
            .ok_or(crate::Error::NotFoundSkill { skill: id })
    }

    pub fn skills(&self) -> &Vec<SkillWithState> {
        &self.inner
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

    pub fn heal_skill_cooldown(
        &mut self,
        target: StaticSkillId,
        heal_num: CooldownNum,
    ) -> Result<(), crate::Error> {
        let target_item = self.get_mut(target)?;

        if target_item.cooldown > heal_num {
            target_item.cooldown -= heal_num;
        } else {
            target_item.cooldown = 0;
        };

        Ok(())
    }

    pub fn heal_skill_cooldown_all(&mut self, heal_num: CooldownNum) {
        for item in self.inner.iter_mut() {
            if item.cooldown > heal_num {
                item.cooldown -= heal_num;
            } else {
                item.cooldown = 0;
            };
        }
    }
}
