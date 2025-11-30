use std::{borrow::Cow, fmt::Debug};

use rand::{Rng, rng};

mod cached_status;
pub mod list;
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
    TriggerTurnStart,
    /// どんなふうに使おうが自由
    ///
    /// 8byteで表現できる情報ならこれで伝えてほしい
    Unique(u64),
    // 必要なら以下を追加する
    //
    // UniqueBuffer([u8; 16]),
    // UniqueBox(std::sync::Arc<dyn std::any::Any>),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum PassiveUpdateStateError {
    #[error("想定していないメッセージ: {0:?}")]
    UnexpectedMessage(PassiveUpdateStateMessage),

    #[error("不正な値のメッセージ")]
    InvalidValue,
}
