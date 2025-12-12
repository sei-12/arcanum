use std::collections::VecDeque;

use crate::{effect::Effect, runtime_id::RuntimeSkillId};

#[derive(Debug, Clone, Copy)]
pub enum ButtleCharCondition {
    Waiting,
    Acting,
    Chanting,
    Preparing,
}

pub struct Action {}

pub struct ButtleChar {
    current_action: Option<Action>,
}

impl ButtleChar {
    pub fn frame(&mut self, effects_buffer: &mut VecDeque<Effect>) {}

    pub fn current_condition(&self) -> ButtleCharCondition {
        todo!()
    }

    pub(crate) fn can_start_skill(&self, skill_id: RuntimeSkillId) -> bool {
        todo!()
    }
}
