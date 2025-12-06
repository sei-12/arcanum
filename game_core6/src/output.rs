use crate::effect::Effect;

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

pub enum GameCoreOutput {
    Effect(Effect),
    Event(Event),
    WaitInput,
}
