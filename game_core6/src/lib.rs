#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod buttle_char;
pub mod buttle_enemy;
pub mod damage;
pub mod effect;
pub mod effector;
pub mod enemy;
pub mod game_core_actor;
pub mod lt_common;
pub mod output;
mod passive;
pub mod potential;
mod sender_side;
mod skill;
pub mod state;

use std::{any::TypeId, collections::VecDeque, sync::Arc};

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

    impl From<RuntimeCharId> for LtId {
        fn from(value: RuntimeCharId) -> Self {
            LtId::Char(value)
        }
    }
    impl From<&RuntimeCharId> for LtId {
        fn from(value: &RuntimeCharId) -> Self {
            LtId::Char(*value)
        }
    }

    impl From<RuntimeEnemyId> for LtId {
        fn from(value: RuntimeEnemyId) -> Self {
            LtId::Enemy(value)
        }
    }
    impl From<&RuntimeEnemyId> for LtId {
        fn from(value: &RuntimeEnemyId) -> Self {
            LtId::Enemy(*value)
        }
    }
}

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
            let output = output_buffer.pop()?;

            if let GameCoreOutput::Effect(_, effect) = &output {
                let result = self.state.accept(effect);
                // output_bufferに入っているエフェクトは全てacceptedがtrueである
                assert!(result.accepted);
            };

            Some(output)
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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("すでにゲームが開始されています")]
    AlreadyGameStart,

    #[error("すでにゲームは終了しています")]
    AlreadyGameEnd,

    #[error("保持していないキャラIDです: got_id={0}")]
    NotFoundChar(StaticCharId),

    #[error("wave数が0です")]
    WavesIsEmpty,

    // #[error("wave内の敵の数が不正です: got={0:?}")]
    // InvalidNumEnemysInWave(Arc<Vec<Vec<EnemyArg>>>),
    #[error("使用できないスキルを使用しようとしています")]
    UnUseableSkill,

    #[error("チームメンバーの数が不正な値です: メンバー数={}", got_num_members)]
    InvalidNumTeamMembers { got_num_members: usize },

    #[error("習得スキル数が不正です")]
    InvalidNumLearnSkills(usize),

    #[error("チーム内に同じキャラクターがいます: id={0}")]
    ConfrictChar(StaticCharId),
}
