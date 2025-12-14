use std::fmt::Debug;

use crate::{
    LevelNum, StaticEnemySkillId,
    buttle_enemy::enemy_skill_runner::EnemySkillRunnner,
    core_actor::CtxContainer,
    enemy_skill::EnemySkill,
    lt_common::LtCommon,
    potential::Potential,
    runtime_id::{LtId, RuntimeEnemyId},
    state::GameState,
};

mod enemy_skill_runner;

pub struct EnemyInfomation {
    pub name: &'static str,
    pub desctiption: &'static str,
}

pub struct ButtleEnemyArgs {
    pub level: LevelNum,
    pub info: EnemyInfomation,
    pub potential: Potential,
    pub skills: Vec<EnemySkill>,
    pub action_patterns: Vec<Vec<StaticEnemySkillId>>,
}

#[derive(Debug)]
pub struct ButtleEnemy {
    id: RuntimeEnemyId,
    lt_common: LtCommon,
    skill_runner: EnemySkillRunnner,
}

impl ButtleEnemy {
    pub(crate) fn new(id: RuntimeEnemyId, args: ButtleEnemyArgs) -> Result<Self, crate::Error> {
        let mut action_patterns = Vec::with_capacity(args.action_patterns.len());
        for pattern in args.action_patterns {
            action_patterns.push(Vec::with_capacity(pattern.len()));
            let last = action_patterns.last_mut().unwrap();
            for skill_id in pattern {
                let Some(idx) = args.skills.iter().position(|s| s.id == skill_id) else {
                    return Err(crate::Error::NotFoundSkillInActionPattern);
                };
                last.push(idx);
            }
        }

        Ok(ButtleEnemy {
            id,
            lt_common: LtCommon::new(args.potential, args.level),
            skill_runner: EnemySkillRunnner::new(args.skills, action_patterns),
        })
    }

    pub fn tick(&mut self) {
        self.skill_runner.tick();
    }

    pub fn frame(&self, state: &GameState, ctx: &mut CtxContainer) {
        self.skill_runner.frame(state, ctx);
        self.lt_common.passive.frame(self.lt_id(), state, ctx);
    }

    pub fn view_skills(&self) -> impl Iterator<Item = &EnemySkill> {
        self.skill_runner.view_skills()
    }

    pub fn runtime_id(&self) -> RuntimeEnemyId {
        self.id
    }

    pub fn lt_id(&self) -> LtId {
        self.runtime_id().into()
    }

    pub fn lt(&self) -> &LtCommon {
        &self.lt_common
    }

    pub(crate) fn lt_mut(&mut self) -> &mut LtCommon {
        &mut self.lt_common
    }
}
