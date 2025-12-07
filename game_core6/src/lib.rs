#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod buttle_char;
pub mod buttle_enemy;
pub mod damage;
pub mod effect;
pub mod effector;
pub mod lt_common;
pub mod output;
pub mod potential;
pub mod state;

use std::{any::TypeId, collections::VecDeque};

use crate::{output::GameCoreOutput, state::GameState};

pub type MpNum = u32;
pub type SpNum = u32;
pub type LevelNum = u32;
pub type StatusNum = f32;
pub type TurnNum = u8;
pub type CooldownNum = u32;
pub type HateNum = u32;
pub type WaveNum = u8;
pub type StaticEnemySkillId = u32;
pub type StaticSkillId = u32;
pub type StaticPassiveId = TypeId;
pub type StaticCharId = u32;
pub type StaticEnemyId = u32;

pub const NUM_MAX_CHAR_IN_TEAM: u8 = 4;
pub const NUM_MAX_LEARN_SKILLS: usize = 256;
pub const NUM_MAX_ENEMYS_IN_WAVE: usize = 5;
pub const TURN_START_HEAL_MP_NUM: MpNum = 100;
pub const SKILL_COOLDOWN_HEAL_BASE: CooldownNum = 50;
pub const TURN_START_HEAL_SP_NUM: SpNum = 50;

pub mod runtime_id {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    pub struct RuntimeCharId {
        pub(crate) idx: u8,
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    pub struct RuntimeEnemyId {
        pub(crate) wave_idx: u8,
        pub(crate) idx: u8,
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    pub struct RuntimeSkillId {
        pub(crate) char_id: RuntimeCharId,
        pub(crate) idx: u8,
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    pub enum LtId {
        Char(RuntimeCharId),
        Enemy(RuntimeEnemyId),
    }
}

pub mod game_core_actor;

trait OutputBuffer {
    fn push(&mut self, item: GameCoreOutput);
    fn pop(&mut self) -> Option<GameCoreOutput>;
}
impl OutputBuffer for VecDeque<GameCoreOutput> {
    fn push(&mut self, item: GameCoreOutput) {
        self.push_back(item);
    }
    fn pop(&mut self) -> Option<GameCoreOutput> {
        self.pop_front()
    }
}

mod skill;

pub mod enemy;
mod passive;
mod sender_side;

mod receiver_side {
    use crate::{OutputBuffer, output::GameCoreOutput, state::GameState};
    use std::collections::VecDeque;

    pub(crate) struct ReceiverSide {
        state: GameState,
    }

    impl ReceiverSide {
        pub(crate) fn forward(
            &mut self,
            output_buffer: &mut impl OutputBuffer,
        ) -> Option<GameCoreOutput> {
            todo!()
        }

        pub(crate) fn state(&self) -> &GameState {
            &self.state
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WinOrLoseOrNextwave {
    Win,
    Lose,
    Nextwave,
}

impl WinOrLoseOrNextwave {
    fn is_win_or_lose(&self) -> bool {
        *self == WinOrLoseOrNextwave::Win || *self == WinOrLoseOrNextwave::Lose
    }
}
