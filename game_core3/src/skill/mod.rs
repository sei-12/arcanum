use std::{fmt::Debug, ops::Deref};

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
    fn initialize_cooldown(&self) -> CooldownNum {
        0
    }
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
pub enum StaticSkill {
    Fireball(fireball::Fireball),
    Waterball(waterball::Waterball),
}

impl StaticSkill {
    fn new(id: StaticSkillId) -> Self {
        match id {
            StaticSkillId::Fireball => StaticSkill::Fireball(fireball::Fireball::new()),
            StaticSkillId::Waterball => StaticSkill::Waterball(waterball::Waterball::new()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticSkillId {
    Fireball,
    Waterball,
}

#[derive(Debug, Clone)]
pub struct SkillWithState {
    static_skill: StaticSkill,
    cooldown: CooldownNum,
}

impl SkillWithState {
    fn new(id: StaticSkillId) -> Self {
        let static_skill = StaticSkill::new(id);
        Self {
            cooldown: static_skill.initialize_cooldown(),
            static_skill,
        }
    }
}

impl Deref for SkillWithState {
    type Target = StaticSkill;
    fn deref(&self) -> &Self::Target {
        &self.static_skill
    }
}
