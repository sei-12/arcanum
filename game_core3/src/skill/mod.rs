use std::{fmt::Debug, ops::Deref};

use crate::{
    CooldownNum, HateNum, MpNum, buttle_char::ButtleChar, event::EventsQuePusher, state::GameState,
};
mod fireball;
// mod waterball;

pub mod skills;

pub struct SkillDocument {
    need_mp: MpNum,
    hate: HateNum,
    cooldown: CooldownNum,
    text: &'static str,
    name: &'static str,
}

#[enum_dispatch::enum_dispatch]
pub trait SkillTrait: Debug + Clone {
    fn id(&self) -> StaticSkillId;
    fn document(&self) -> &'static SkillDocument;
    fn call(
        &self,
        user: &ButtleChar,
        state: &GameState,
        events: &mut impl EventsQuePusher,
    ) -> SkillResult;

    fn useable(&self, user: &ButtleChar, state: &GameState) -> bool {
        let cooldown = user.skills.get(self.id()).unwrap().cooldown == 0;
        let mp = self.need_mp(user, state) <= state.player_mp();
        cooldown && mp
    }

    #[allow(unused_variables)]
    fn need_mp(&self, user: &ButtleChar, state: &GameState) -> MpNum {
        self.document().need_mp
    }

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
        // events.push(Eve);
    }
}

#[derive(Debug, Clone)]
#[enum_dispatch::enum_dispatch(SkillTrait)]
pub enum StaticSkill {
    Fireball(fireball::Fireball),
    // Waterball(waterball::Waterball),
}

impl StaticSkill {
    fn new(id: StaticSkillId) -> Self {
        match id {
            StaticSkillId::Fireball => StaticSkill::Fireball(fireball::Fireball::new()),
            // StaticSkillId::Waterball => StaticSkill::Waterball(waterball::Waterball::new()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticSkillId {
    Fireball,
    // Waterball,
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
