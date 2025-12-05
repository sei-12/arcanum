// use crate::{
//     StaticSkillId,
//     state::{GameState, RuntimeCharId, UpdateStateMessage},
// };

// pub trait SkillFunction: Copy {
//     fn call(&self, user_id: RuntimeCharId, effector: &mut dyn Effector);
// }
// pub trait PassiveFunction: Copy {}

// pub(crate) enum CoreMessage {
//     Effect(EffectedBy, UpdateStateMessage),
//     Event(Event),
//     WaitUserInput,
// }

// pub enum EffectedBy {
//     CharSkill(StaticSkillId),
//     EnemySkill,
//     SubEffect,
//     GameSystem,
// }

// pub enum Effect {
//     Damage,
//     HealHp,
//     ConsumeMp,
//     HealMp,
//     ConsumeSp,
//     HealSp,
//     AddPassive,
//     SetSkillCooldown,
//     HealSkillCooldown,
//     HealSkillCooldownAll,
//     AddHate,
// }
// impl TryFrom<UpdateStateMessage> for Effect {
//     type Error = ();
//     fn try_from(value: UpdateStateMessage) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }

// pub enum Event {
//     CharUseSkill,
//     EnemyUseSkill,
//     PlayerTurnStart,
//     EnemyTurnStart,
//     GoNextWave,
//     Win,
//     Lose,
//     DeadEnemy,
// }

// pub enum GameCoreOutput {
//     Effect(EffectedBy, Effect),
//     Event(Event),
//     WaitUserInput,
// }

// pub trait Effector {}

// pub trait MessageReceiver2 {
//     fn unblock_recv(&mut self) -> Result<Option<CoreMessage>, Box<dyn std::error::Error>>;
// }
// // todo rename
// pub struct GameCoreOutputReceiver2<R: MessageReceiver2> {
//     receiver: R,
//     state: GameState,
// }

// impl<R: MessageReceiver2> GameCoreOutputReceiver2<R> {
//     pub fn forword(&mut self) -> Result<Option<GameCoreOutput>, Box<dyn std::error::Error>> {
//         let Some(msg) = self.receiver.unblock_recv()? else {
//             return Ok(None);
//         };

//         let (by, message) = match msg {
//             CoreMessage::Event(e) => {
//                 return Ok(Some(GameCoreOutput::Event(e)));
//             }
//             CoreMessage::WaitUserInput => {
//                 return Ok(Some(GameCoreOutput::WaitUserInput));
//             }
//             CoreMessage::Effect(by, effect) => (by, effect),
//         };

//         self.state.update(&message);

//         let Some(output_effect) = Effect::try_from(message).ok() else {
//             return Ok(None);
//         };

//         Ok(Some(GameCoreOutput::Effect(by, output_effect)))
//     }
// }
