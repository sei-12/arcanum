use std::fmt::Debug;

use crate::{
    LevelNum, StaticEnemySkillId, buttle_enemy::enemy_skill_runner::EnemySkillRunnner,
    core_actor::CtxContainer, enemy_skill::EnemySkill, lt_common::LtCommon,
    passive::passive_box::PassiveBox, potential::Potential, runtime_id::LtId, state::GameState,
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
    pub default_passive: Vec<PassiveBox>,
}

#[derive(Debug)]
pub struct ButtleEnemy {
    lt_common: LtCommon,
    skill_runner: EnemySkillRunnner,
}

impl ButtleEnemy {
    pub(crate) fn new(args: ButtleEnemyArgs) -> Result<Self, crate::Error> {
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

        let mut lt_common = LtCommon::new(args.potential, args.level);
        args.default_passive.into_iter().for_each(|p| {
            lt_common.passive.add(p);
        });

        Ok(ButtleEnemy {
            lt_common,
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

    pub fn lt_id(&self) -> LtId {
        LtId::Enemy
    }

    pub fn lt(&self) -> &LtCommon {
        &self.lt_common
    }

    pub(crate) fn lt_mut(&mut self) -> &mut LtCommon {
        &mut self.lt_common
    }
}
