use std::{any::Any, cell::Ref, collections::VecDeque, fmt::Debug};

pub mod passive_box;

use crate::{
    StaticPassiveId, any_message::AnyMessage, buff_status::BuffStatus, effect::Effect,
    passive::passive_box::PassiveBox, runtime_id::LtId, state::GameState,
};

pub struct PassiveInfo {
    pub name: &'static str,
    pub description: &'static str,
}

pub trait PassiveTrait: Debug {
    fn display(&self) -> String;
    fn clone_box(&self) -> PassiveBox;
    fn static_id(&self) -> StaticPassiveId;
    fn merge(&mut self, passive: &dyn Any);
    fn should_trash(&self) -> bool;
    fn update(&mut self, msg: &AnyMessage);
    fn info(&self) -> &PassiveInfo;
    fn frame(&self, owner: LtId, state: &GameState, effects_buffer: &mut VecDeque<Effect>);
    fn status(&self, s: BuffStatus);
}

#[derive(Debug, Clone, Default)]
pub struct PassiveList {}
impl PassiveList {
    pub(crate) fn frame(&self, state: &GameState, effects_buffer: &mut VecDeque<Effect>) {}
    pub(crate) fn status(&self) -> Ref<'_, BuffStatus> {
        todo!()
    }
}
