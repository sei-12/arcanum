use crate::{StaticEnemySkillId, StaticPassiveId, StaticSkillId, effect::Effect};

pub enum Event {
    CharUseSkill,
    EnemyUseSkill,
    PlayerTurnStart,
    EnemyTurnStart,
    GoNextWave,
    Win,
    Lose,
    DeadEnemy,
}

#[derive(Debug, Clone, Copy)]
pub enum EffectedBy {
    CharSkill(StaticSkillId),
    EnemySkill(StaticEnemySkillId),
    GameSystem,
    SubEffect(StaticPassiveId),
}

pub enum GameCoreOutput {
    Effect(EffectedBy, Effect),
    Event(Event),
    WaitInput,
}
