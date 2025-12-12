use std::collections::VecDeque;

use crate::{
    CooldownNum,
    effect::Effect,
    lt_common::LtCommon,
    runtime_id::{RuntimeCharId, RuntimeSkillId},
    skill::{SkillBox, UsingSkillState},
    state::GameState,
    weapon::{Weapon, WeaponType},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtleCharCondition {
    Waiting,
    Acting,
    Chanting,
    Preparing,
}

#[derive(Debug, Clone)]
pub struct Action {
    skill_id: RuntimeSkillId,
    state: UsingSkillState,
}

pub struct ButtleChar {
    id: RuntimeCharId,
    current_action: Option<Action>,
    skills: Vec<ButtleSkill>,
    lt_common: LtCommon,
    weapon: Weapon,
}

impl ButtleChar {
    pub fn frame(&mut self, state: &GameState, effects_buffer: &mut VecDeque<Effect>) {
        if let Some(action) = &self.current_action {
            self.get_skill(action.skill_id).static_data.frame(
                self.runtime_id(),
                state,
                &action.state,
                effects_buffer,
            );
        }

        self.lt_common.passive.frame(state, effects_buffer);

        effects_buffer.push_back(Effect::HealSkillCooldownAll {
            target_id: self.runtime_id(),
            num: self.cooldown_heal(),
        });
        effects_buffer.push_back(Effect::HealMp {
            num: self.lt().mp_heal(),
        });
    }

    pub fn current_condition(&self) -> ButtleCharCondition {
        let Some(action) = &self.current_action else {
            return ButtleCharCondition::Waiting;
        };

        self.get_skill(action.skill_id)
            .static_data
            .current_condition(&action.state)
    }

    pub(crate) fn spawn_skill_action(&mut self, skill_id: RuntimeSkillId, state: UsingSkillState) {
        assert!(self.current_action.is_none());
        assert!(self.can_start_skill(skill_id));
        self.current_action = Some(Action { skill_id, state });
    }

    pub(crate) fn can_start_skill(&self, skill_id: RuntimeSkillId) -> bool {
        self.get_skill(skill_id).cooldown <= 0.0
            && self.current_condition() == ButtleCharCondition::Waiting
    }

    pub(crate) fn get_skill(&self, id: RuntimeSkillId) -> &ButtleSkill {
        todo!()
    }

    pub fn runtime_id(&self) -> RuntimeCharId {
        self.id
    }

    pub fn get_skills(&self) -> &Vec<ButtleSkill> {
        &self.skills
    }

    pub fn weapon_type(&self) -> WeaponType {
        self.weapon.ty
    }

    pub fn lt(&self) -> &LtCommon {
        &self.lt_common
    }

    pub(crate) fn lt_mut(&mut self) -> &mut LtCommon {
        todo!()
    }

    pub fn cooldown_heal(&self) -> CooldownNum {
        todo!()
    }
}

pub struct ButtleSkill {
    cooldown: CooldownNum,
    static_data: SkillBox,
}

impl ButtleSkill {}
