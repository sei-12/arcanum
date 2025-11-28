use std::fmt::Debug;

use crate::{
    CooldownNum, HateNum, MpNum, buttle_char::ButtleChar, event::EventsQuePusher, state::GameState,
};
mod fireball;
mod waterball;

pub mod skills;

#[enum_dispatch::enum_dispatch]
pub trait SkillTrait: Debug + Clone {
    fn id(&self) -> StaticSkillId;
    fn name(&self) -> &'static str;
    fn text(&self) -> &'static str;
    fn call(
        &self,
        user: &ButtleChar,
        state: &GameState,
        events: &mut impl EventsQuePusher,
    ) -> SkillResult;
    fn useable(&self, user: &ButtleChar, state: &GameState);
}

pub struct SkillResult {
    consume_mp: MpNum,
    hate: HateNum,
    cooldown: CooldownNum,
}
impl SkillResult {
    pub(crate) fn to_events(&self, events: &mut impl EventsQuePusher) {
        todo!()
    }
}

#[derive(Debug, Clone)]
#[enum_dispatch::enum_dispatch(SkillTrait)]
pub enum Skill {
    Fireball(fireball::Fireball),
    Waterball(waterball::Waterball),
}

impl Skill {
    pub fn new(id: StaticSkillId) -> Self {
        match id {
            StaticSkillId::Fireball => Skill::Fireball(fireball::Fireball::new()),
            StaticSkillId::Waterball => Skill::Waterball(waterball::Waterball::new()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticSkillId {
    Fireball,
    Waterball,
}
