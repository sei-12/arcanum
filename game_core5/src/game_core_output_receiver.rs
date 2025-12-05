use crate::{
    MessageReceiver, StaticSkillId,
    effect::Effect,
    state::{GameState, RuntimeCharId},
};

pub trait SkillFunction: Copy {
    fn call(&self, user_id: RuntimeCharId, effector: &mut dyn Effector);
}
pub trait PassiveFunction: Copy {}

// GameCoreOutputと同じだけど統一しない
// 改めて考え直して統一してもいい
#[derive(Debug, Clone)]
pub(crate) enum CoreMessage {
    Effect(EffectedBy, Effect),
    Event(Event),
    WaitUserInput,
}

#[derive(Debug, Clone)]
pub enum EffectedBy {
    CharSkill(StaticSkillId),
    EnemySkill,
    SubEffect,
    GameSystem,
}

#[derive(Debug, Clone)]
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
    Effect(EffectedBy, Effect),
    Event(Event),
    WaitUserInput,
}

pub trait Effector {}

pub struct GameCoreOutputReceiver<R: MessageReceiver> {
    receiver: R,
    state: GameState,
}

impl<R: MessageReceiver> GameCoreOutputReceiver<R> {
    pub(crate) fn new(receiver: R, state: GameState) -> Self {
        Self { receiver, state }
    }

    pub fn forword(&mut self) -> Result<Option<GameCoreOutput>, Box<dyn std::error::Error>> {
        let Some(msg) = self.receiver.unblock_recv()? else {
            return Ok(None);
        };

        let (by, message) = match msg.inner {
            CoreMessage::Event(e) => {
                return Ok(Some(GameCoreOutput::Event(e)));
            }
            CoreMessage::WaitUserInput => {
                return Ok(Some(GameCoreOutput::WaitUserInput));
            }
            CoreMessage::Effect(by, effect) => (by, effect),
        };

        self.state.accept_effect(&message);

        Ok(Some(GameCoreOutput::Effect(by, message)))
    }
}
