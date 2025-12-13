use crate::runtime_id::{RuntimeCharId, RuntimeSkillId};

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

    #[error("スキルが見当たりませんでした: id={0:?}")]
    NotFoundSkill(RuntimeSkillId),

    #[error("キャラクターが見当たりませんでした: id={0:?}")]
    NotFoundChar(RuntimeCharId),

    #[error("敵キャラクターが見当たりませんでした: id={0:?}")]
    NotFoundEnemy(RuntimeCharId),

    #[error("敵キャラクター数が不正です: num_enemys={0}")]
    InvalidNumEnemys(usize),
    // #[error("チーム内に同じキャラクターがいます: id={0}")]
    // ConfrictChar(StaticCharId),
}
