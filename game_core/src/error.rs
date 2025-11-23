#[derive(Debug, PartialEq)]
pub enum GameError {
    InvalidSkillId,
    InvalidCharId,
    InvalidEnemyId,
    ConfrictChar,
    NotEnoughMp,
    GameEnded,
}
