use std::collections::VecDeque;

use crate::{
    LevelNum, StaticCharId, StatusNum, TimeNum,
    buttle_skill::ButtleSkill,
    effect::Effect,
    game_state::GameState,
    lt_common::LtCommon,
    potential::Potential,
    progress_state::ProgressState,
    runtime_id::{LtId, RuntimeCharId, RuntimeSkillId},
    skill::{CharSkillProgress, SkillBox},
    weapon::{Weapon, WeaponType},
};

pub struct ButtleCharArgs {
    pub static_id: StaticCharId,
    pub potential: Potential,
    pub skills: Vec<SkillBox>,
    pub weapon: Weapon,
    pub level: LevelNum,
    pub name: &'static str,
}

#[derive(Debug)]
pub struct ButtleChar {
    lt_common: LtCommon,
    current_using_skill: Option<RuntimeSkillId>,
    skills: Vec<ButtleSkill>,
    weapon: Weapon,
    runtime_id: RuntimeCharId,
    hate: StatusNum,
    static_id: StaticCharId,
    name: &'static str,
}

impl ButtleChar {
    pub fn new(runtime_id: RuntimeCharId, args: ButtleCharArgs) -> Result<Self, crate::Error> {
        let lt_common = LtCommon::new_with_weapon(args.potential, args.level, args.weapon.clone());

        let mut skills = Vec::with_capacity(args.skills.len());
        for (i, s) in args.skills.into_iter().enumerate() {
            let id = RuntimeSkillId {
                char_id: runtime_id,
                idx: i as u8,
            };
            skills.push(ButtleSkill::new(id, s));
        }

        Ok(Self {
            lt_common,
            current_using_skill: None,
            skills,
            weapon: args.weapon,
            hate: 0.0,
            runtime_id,
            static_id: args.static_id,
            name: args.name,
        })
    }

    pub(crate) fn use_skill(&mut self, id: RuntimeSkillId) {
        self.current_using_skill = Some(id);
        self.get_skill_mut(id).skill_box_mut().start();
    }

    pub fn lt(&self) -> &LtCommon {
        &self.lt_common
    }

    pub(crate) fn lt_mut(&mut self) -> &mut LtCommon {
        &mut self.lt_common
    }

    pub fn runtime_id(&self) -> RuntimeCharId {
        self.runtime_id
    }

    pub fn lt_id(&self) -> LtId {
        self.runtime_id().into()
    }

    pub(crate) fn tick(&self, state: &GameState, effects_buffer: &mut VecDeque<Effect>) {
        if let Some(skill_id) = self.current_using_skill {
            let skill = self.skills.get(skill_id.idx as usize).unwrap();
            skill
                .skill_box()
                .tick(self.runtime_id, state, effects_buffer);
        }

        self.lt_common.tick(self.lt_id(), state, effects_buffer);
        effects_buffer.push_back(Effect::HealSkillCooldownAll {
            target_id: self.runtime_id,
            num: 0.01, // 1 = 1s
        });
    }

    pub fn get_skill(&self, id: RuntimeSkillId) -> &ButtleSkill {
        debug_assert_eq!(self.runtime_id, id.char_id);
        self.skills.get(id.idx as usize).unwrap()
    }

    pub fn get_skills(&self) -> &Vec<ButtleSkill> {
        &self.skills
    }

    pub(crate) fn get_skill_mut(&mut self, id: RuntimeSkillId) -> &mut ButtleSkill {
        self.skills.get_mut(id.idx as usize).unwrap()
    }

    pub(crate) fn heal_skill_cooldown_all(&mut self, num: TimeNum) {
        self.skills.iter_mut().for_each(|s| {
            s.heal_cooldown(num);
        });
    }

    pub(crate) fn add_hate(&mut self, num: StatusNum) {
        self.hate += num;
    }

    pub fn hate(&self) -> StatusNum {
        self.hate
    }

    pub fn weapon_type(&self) -> WeaponType {
        self.weapon.ty
    }
    pub fn static_id(&self) -> StaticCharId {
        self.static_id
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn current_condition(&self) -> CharCondition {
        let skill_progress = self.current_using_skill.map(|s| {
            self.get_skill(s)
                .skill_box()
                .current_progress()
                .expect("使用中のスキルはSomeを返す")
        });

        match skill_progress {
            Some(s) => CharCondition::UseSkill(s),
            None => CharCondition::Wait,
        }
    }
}

pub enum CharCondition {
    UseSkill(CharSkillProgress),
    Wait,
}
