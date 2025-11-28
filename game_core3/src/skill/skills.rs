use crate::skill::{Skill, SkillTrait, StaticSkillId};

#[derive(Debug, Clone)]
pub struct ButtleSkills {
    inner: Vec<Skill>,
}

impl ButtleSkills {
    pub fn new(skill_ids: &[StaticSkillId]) -> Self {
        todo!()
    }

    pub fn get(&self, id: StaticSkillId) -> Result<&Skill, crate::Error> {
        self.inner
            .iter()
            .find(|skill| skill.id() == id)
            .ok_or(crate::Error::NotFoundSkill { skill: id })
    }

}
