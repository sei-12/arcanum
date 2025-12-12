use std::{any::Any, collections::VecDeque};

use crate::{
    StaticPassiveId, any_message::AnyMessage, buff_status::BuffStatus, effect::Effect,
    runtime_id::LtId, state::GameState,
};

pub struct PassiveInfo {
    pub name: &'static str,
    pub description: &'static str,
}

pub trait PassiveTrait {
    fn display(&self) -> String;
    fn static_id(&self) -> StaticPassiveId;
    fn merge(&mut self, buffer: &mut dyn Any);
    fn should_trash(&self) -> bool;
    fn update(&mut self, msg: &AnyMessage);
    fn info(&self) -> &PassiveInfo;
    fn frame(&self, owner: LtId, state: &GameState, effects_buffer: &mut VecDeque<Effect>);
    fn status(&self, s: BuffStatus);
}

pub struct PassiveList {}
impl PassiveList {
    pub(crate) fn frame(&self, effects_buffer: &mut VecDeque<Effect>) {}
}
