use std::{fmt::Debug, ops::Deref};

use crate::{
    CooldownNum, HateNum, MpNum,
    buttle_char::ButtleChar,
    event::EventsQuePusher,
    state::{GameState, chars::RuntimeCharId},
};
mod fireball;

pub mod skills;

pub struct SkillDocument {
    pub need_mp: MpNum,
    pub hate: HateNum,
    pub cooldown: CooldownNum,
    pub text: &'static str,
    pub name: &'static str,
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
    pub(crate) fn to_events(
        &self,
        events: &mut impl EventsQuePusher,
        user_id: RuntimeCharId,
        skill_id: StaticSkillId,
    ) {
        events.push(crate::event::Event::ConsumeMp {
            mp: self.consume_mp,
        });
        events.push(crate::event::Event::AddHate {
            char_id: user_id,
            hate: self.hate,
        });

        events.push(crate::event::Event::SetSkillCooldown {
            char_id: user_id,
            skill_id,
            cooldown: self.cooldown,
        });
    }
}

#[derive(Debug, Clone)]
#[enum_dispatch::enum_dispatch(SkillTrait)]
pub enum StaticSkill {
    Fireball(fireball::Fireball),
}

impl StaticSkill {
    fn new(id: StaticSkillId) -> Self {
        match id {
            StaticSkillId::Fireball => StaticSkill::Fireball(fireball::Fireball::new()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaticSkillId {
    Fireball,
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
    
    pub fn static_data(&self) -> &StaticSkill {
        &self.static_skill
    }

    pub fn cooldown(&self) -> &CooldownNum {
        &self.cooldown
    }
}

impl Deref for SkillWithState {
    type Target = StaticSkill;
    fn deref(&self) -> &Self::Target {
        &self.static_skill
    }
}
