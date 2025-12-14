use crate::{
    CooldownNum, HateNum, LevelNum, StaticCharId,
    any_message::AnyMessage,
    core_actor::CtxContainer,
    effect::Effect,
    lt_common::LtCommon,
    passive::passive_box::PassiveBox,
    potential::Potential,
    runtime_id::{LtId, RuntimeCharId, RuntimeSkillId},
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

#[derive(Debug)]
pub struct ButtleChar {
    static_id: StaticCharId,
    runtime_id: RuntimeCharId,
    current_action: Option<Action>,
    skills: Vec<ButtleSkill>,
    lt_common: LtCommon,
    weapon: Weapon,
    hate: HateNum,
}

#[derive(Debug, Clone)]
pub struct ButtleCharArgs {
    pub static_id: StaticCharId,
    pub name: &'static str,
    pub level: LevelNum,
    pub potential: Potential,
    pub default_passives: Vec<PassiveBox>,
    pub skills: Vec<SkillBox>,
    pub weapon: Weapon,
}

impl ButtleChar {
    pub(crate) fn new(
        runtime_id: RuntimeCharId,
        data: ButtleCharArgs,
    ) -> Result<Self, crate::Error> {
        if data.skills.is_empty() {
            return Err(crate::Error::InvalidNumLearnSkills(data.skills.len()));
        }

        let mut lt_common =
            LtCommon::new_with_weapon(data.potential.clone(), data.level, data.weapon.clone());

        data.default_passives.into_iter().for_each(|p| {
            lt_common.passive.add(p);
        });

        let skills = data
            .skills
            .into_iter()
            .map(|s| ButtleSkill {
                cooldown: 0.0,
                static_data: s,
            })
            .collect::<Vec<_>>();

        Ok(Self {
            static_id: data.static_id,
            runtime_id,
            current_action: None,
            skills,
            lt_common,
            weapon: data.weapon,
            hate: 0.0,
        })
    }

    pub fn frame(&self, state: &GameState, ctx: &mut CtxContainer) {
        if let Some(action) = &self.current_action {
            self.get_skill(action.skill_id).static_data.frame(
                self.runtime_id(),
                state,
                &action.state,
                ctx,
            );
        }

        self.lt_common.passive.frame(self.lt_id(), state, ctx);

        ctx.effects_buffer.push_back(Effect::HealSkillCooldownAll {
            target_id: self.runtime_id(),
            num: 1.0,
        });

        ctx.effects_buffer.push_back(Effect::HealMp {
            target_id: self.lt_id(),
            num: self.lt().mp_heal(),
        });
    }

    pub fn lt_id(&self) -> LtId {
        LtId::Char(self.runtime_id())
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

    pub(crate) fn try_get_skill(&self, id: RuntimeSkillId) -> Result<&ButtleSkill, crate::Error> {
        self.skills
            .get(id.idx as usize)
            .ok_or(crate::Error::NotFoundSkill(id))
    }

    pub(crate) fn get_skill(&self, id: RuntimeSkillId) -> &ButtleSkill {
        self.try_get_skill(id).unwrap()
    }

    pub fn static_id(&self) -> StaticCharId {
        self.static_id
    }

    pub fn runtime_id(&self) -> RuntimeCharId {
        self.runtime_id
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
        &mut self.lt_common
    }

    fn get_skill_mut(&mut self, id: RuntimeSkillId) -> &mut ButtleSkill {
        self.skills.get_mut(id.idx as usize).unwrap()
    }

    pub(crate) fn add_skill_cooldown(&mut self, skill_id: RuntimeSkillId, num: CooldownNum) {
        let skill = self.get_skill_mut(skill_id);
        skill.cooldown += num;
    }

    pub(crate) fn heal_skill_cooldown(&mut self, skill_id: RuntimeSkillId, num: CooldownNum) {
        let skill = self.get_skill_mut(skill_id);
        skill.cooldown -= num;
    }

    pub(crate) fn heal_skill_cooldown_all(&mut self, num: CooldownNum) {
        self.skills.iter_mut().for_each(|s| s.cooldown -= num);
    }

    pub(crate) fn add_hate(&mut self, num: HateNum) {
        self.hate += num;
    }

    pub(crate) fn heal_hate(&mut self, num: HateNum) {
        self.hate -= num;
    }

    pub fn hate(&self) -> HateNum {
        self.hate
    }

    pub(crate) fn update_skill_state(&mut self, skill_id: RuntimeSkillId, msg: &AnyMessage) {
        self.skills[skill_id.idx as usize].static_data.update(msg);
    }

    pub(crate) fn tick(&mut self) {
        if let Some(action) = &mut self.current_action {
            action.state.tick(&self.lt_common);
        }
    }
}

#[derive(Debug)]
pub struct ButtleSkill {
    cooldown: CooldownNum,
    static_data: SkillBox,
}

impl ButtleSkill {
    pub fn cooldown(&self) -> CooldownNum {
        if self.cooldown < 0.0 {
            0.0
        } else {
            self.cooldown
        }
    }

    pub fn static_data(&self) -> &SkillBox {
        &self.static_data
    }
}
