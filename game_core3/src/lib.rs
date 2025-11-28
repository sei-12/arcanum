pub mod event;
pub mod skill;
pub mod state;
pub mod static_char;
pub mod potential;
pub mod lt_common;
pub mod passive;
pub mod buttle_char;
pub mod args;
pub mod error;
pub mod screen_actor;
pub mod container;
pub mod buttle_enemy;
pub mod damage;
pub mod enemys;

pub use error::Error;

pub type MpNum = u32;
pub type LevelNum = u32;
pub type StatusNum = f32;
pub type TurnNum = u8;
pub type CooldownNum = u32;
pub type HateNum = u32;
pub type WaveNum = u8;

pub const NUM_MAX_CHAR_IN_TEAM: usize = 4;
pub const NUM_MAX_LEARN_SKILLS: usize = 6;
pub const TURN_START_HEAL_MP_NUM: MpNum = 100;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Win,
    Lose,
}