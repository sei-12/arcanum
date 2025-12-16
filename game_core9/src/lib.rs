pub mod core_actor;
pub mod error;
pub mod runtime_id;
// pub mod accepter;
mod any_message;
pub mod buttle_char;
pub mod buttle_enemy;
pub mod buttle_skill;
pub mod damage;
pub mod effect;
pub mod enemy_skill;
pub mod game_state;
pub mod lt_common;
pub mod passive;
pub mod potential;
pub mod skill;
pub mod weapon;

pub use error::Error;

pub type StaticEnemySkillId = u32;
pub type StaticCharId = u32;
pub type StaticSkillId = u32;
pub type StaticPassiveId = u32;
pub type StatusNum = f32;
pub type TimeNum = f32;
pub type LevelNum = u32;

pub const MAX_CHARACTERS: usize = 4;
