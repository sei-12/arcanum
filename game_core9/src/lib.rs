pub mod core_actor;
pub mod error;
pub mod runtime_id;
// pub mod accepter;
pub mod game_state;
pub mod buttle_char;
pub mod effect;
pub mod buttle_enemy;
mod any_message;
pub mod skill;
pub mod buttle_skill;
pub mod damage;
pub mod lt_common;
pub mod potential;
pub mod passive;

pub use error::Error;

pub type StaticSkillId = u32;
pub type StaticPassiveId = u32;
pub type StatusNum = f32;
pub type TimeNum = f32;
pub type LevelNum = u32;
