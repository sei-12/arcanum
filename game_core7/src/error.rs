#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("すでにゲームが開始されています")]
    AlreadyGameStart,

    #[error("すでにゲームは終了しています")]
    AlreadyGameEnd,

    // #[error("保持していないキャラIDです: got_id={0}")]
    // NotFoundChar(StaticCharId),

    // #[error("wave数が不正です len={0}")]
    // InvalidNumWaves(usize),

    // #[error("wave内の敵の数が不正です: got={0:?}")]
    // InvalidNumEnemysInWave(DungeonData),
    #[error("使用できないスキルを使用しようとしています")]
    UnUseableSkill,

    #[error("チームメンバーの数が不正な値です: メンバー数={0}")]
    InvalidNumTeamMembers(usize),

    #[error("習得スキル数が不正です")]
    InvalidNumLearnSkills(usize),
    // #[error("チーム内に同じキャラクターがいます: id={0}")]
    // ConfrictChar(StaticCharId),
}
