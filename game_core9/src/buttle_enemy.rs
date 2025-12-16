// use std::collections::VecDeque;

// use crate::{effect::Effect, game_state::GameState, lt_common::LtCommon, runtime_id::LtId};

// pub struct ButtleEnemyArgs {}

// pub struct ButtleEnemy {
//     lt_common: LtCommon,
// }

// impl ButtleEnemy {
//     pub fn new(args: ButtleEnemyArgs) -> Self {
//         todo!()
//     }

//     pub fn tick(&self, state: &GameState, effects_buffer: &mut VecDeque<Effect>) {
//         self.lt_common.tick(self.lt_id(), state, effects_buffer);
//     }

//     pub fn lt_id(&self) -> LtId {
//         LtId::Enemy
//     }

//     pub fn lt_mut(&mut self) -> &mut LtCommon {
//         &mut self.lt_common
//     }

//     pub fn lt(&self) -> &LtCommon {
//         &self.lt_common
//     }
// }

use std::{collections::VecDeque, fmt::Debug};

use crate::{
    LevelNum, StaticEnemySkillId, buttle_enemy::enemy_skill_runner::EnemySkillRunnner,
    effect::Effect, enemy_skill::EnemySkill, game_state::GameState, lt_common::LtCommon,
    passive::PassiveBox, potential::Potential, runtime_id::LtId,
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

    pub(crate) fn tick(&self, state: &GameState, effects_buffer: &mut VecDeque<Effect>) {
        self.lt_common.tick(self.lt_id(), state, effects_buffer);
        self.skill_runner.tick(state, effects_buffer);
    }

    pub(crate) fn skill_runner_increment_frame(&mut self) {
        self.skill_runner.increment_frame();
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
