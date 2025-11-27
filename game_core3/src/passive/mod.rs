use std::{
    any::{self, Any, type_name, type_name_of_val},
    borrow::Cow,
    fmt::Debug,
};

use dyn_clone::DynClone;
use rand::{Rng, rng};

use crate::{
    buttle_char::RuntimeCharId,
    passive::status::PassiveStatus,
    state::{GameState, LtId},
};

// mod cached_status;
// mod effect;
// pub mod list;
// pub mod public_passive;
pub mod passive_events;
pub mod status;
pub mod traits;

pub type PassiveRuntimeId = u32;
pub(crate) fn gen_passive_runtime_id() -> PassiveRuntimeId {
    let mut rng = rng();
    rng.random()
}

#[derive(Debug, Clone)]
pub struct DisplayPassiveInfo<'a> {
    pub header: Cow<'a, str>,
    pub text: Cow<'a, str>,
}

#[derive(Debug, Clone)]
pub enum PassiveUpdateStateMessage {
    DecrimentTurns,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum PassiveUpdateStateError {
    #[error("想定していないメッセージ: {0:?}")]
    UnexpectedMessage(PassiveUpdateStateMessage),
}
