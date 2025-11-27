use crate::{
    event::{Event, EventsQue},
    state::GameState,
};

mod fireball;
mod waterball;

#[enum_dispatch::enum_dispatch]
pub trait SkillTrait {
    fn name(&self) -> &'static str;
    fn text(&self) -> &'static str;
    fn call(&self, state: &GameState, events: &mut impl EventsQue);
}

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

pub enum StaticSkillId {
    Fireball,
    Waterball,
}
