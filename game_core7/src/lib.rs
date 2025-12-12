#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::runtime_id::{RuntimeCharId, RuntimeSkillId};

pub mod any_message;
pub mod buttle_char;
pub mod core_actor;
pub mod damage;
pub mod effect;
pub mod output;
pub mod passive;
pub mod state;
pub mod skill;
pub mod buff_status;
pub mod lt_common;
pub mod potential;
pub mod weapon;
pub mod buttle_enemy;

pub type MpNum = f32;
// pub type SpNum = u32;
pub type LevelNum = u32;
pub type StatusNum = f32;
// pub type TurnNum = u8;
pub type CooldownNum = f32;
pub type TimeNum = f32;
pub type HateNum = f32;
// pub type WaveNum = u8;
pub type StaticEnemySkillId = u32;
pub type StaticSkillId = u32;
pub type StaticPassiveId = u32;
pub type StaticCharId = u32;
pub type StaticEnemyId = u32;

pub enum UserInput {
    UseSkill {
        char_id: RuntimeCharId,
        skill_id: RuntimeSkillId,
    },
    None,
}

pub mod runtime_id {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    pub struct RuntimeCharId {
        pub(crate) idx: u8,
    }

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    pub struct RuntimeEnemyId {
        // pub(crate) wave_idx: u8,
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
