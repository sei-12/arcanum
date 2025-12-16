use std::collections::VecDeque;

use crate::{
    StaticEnemySkillId, StatusNum, damage::DamageType, effect::Effect, game_state::GameState,
    passive::PassiveBox,
};

#[derive(Debug, Clone)]
pub struct EnemySkill {
    pub id: StaticEnemySkillId,
    pub name: &'static str,
    pub need_mp: f32,
    pub start_up_frames: u64,
    pub recovery_frame: u64,
    pub actions: Vec<(EnemySkillTarget, EnemySkillAction)>,
}

impl EnemySkill {
    pub(crate) fn run_actions(&self, state: &GameState, effects_buffer: &mut VecDeque<Effect>) {
        todo!()
    }

    pub(crate) fn total_frames(&self) -> u64 {
        self.start_up_frames + self.recovery_frame
    }
}

#[derive(Debug, Clone)]
pub enum EnemySkillTarget {
    Self_,
    Single,
    Multi(u8),
    All,
}

#[derive(Debug, Clone)]
pub enum EnemySkillAction {
    Damage {
        ty: DamageType,
        dmg_mag: StatusNum,
        count: u8,
    },
    AddPassive(PassiveBox),
}
