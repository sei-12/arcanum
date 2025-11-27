use crate::static_char::StaticCharId;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("すでにゲームが開始されています")]
    AlreadyGameStart,

    #[error("保持していないキャラIDです: got_id={0}")]
    NotFoundChar(StaticCharId),

    #[error("チームメンバーの数が不正な値です: メンバー数={}", got_num_members)]
    InvalidNumTeamMembers { got_num_members: usize },

    #[error("チーム内に同じキャラクターがいます: id={0}")]
    ConfrictChar(StaticCharId),
}
