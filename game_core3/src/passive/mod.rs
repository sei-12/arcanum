use std::{borrow::Cow, fmt::Debug};

use rand::{Rng, rng};

mod cached_status;
pub mod list;
// pub mod passive_events;
pub(crate) mod public_passive;
pub mod status;
pub mod traits;

pub type RuntimePassiveId = u32;
pub(crate) fn gen_passive_runtime_id() -> RuntimePassiveId {
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
