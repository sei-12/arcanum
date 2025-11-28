use crate::{enemys::RuntimeEnemyId, event::EventsQuePusher, state::GameState};

pub const SKILL_NAME: &str = "ひっかく";
pub fn call(enemy: RuntimeEnemyId, state: &GameState, events: &mut impl EventsQuePusher) {}
