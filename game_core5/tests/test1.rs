// use std::sync::Arc;

// use game_core5::{
//     ButtleArgs, CharArg, CoreOutput, EnemyArg, EnemySkill, StaticCharData, StaticEnemyData,
//     TURN_START_HEAL_MP_NUM, WinOrLoseOrNextwave,
//     effector::Effecter,
//     living_thing::Potential,
//     skill::{SkillCost, StaticSkillData},
//     state::{GameState, RuntimeCharId, RuntimeEnemyId},
// };

// use crate::chunnel::chunnel;

// const CHAR: StaticCharData = StaticCharData {
//     id: 1,
//     name: "hello",
//     potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
// };

// fn skill_fn(
//     _user: RuntimeCharId,
//     _target: Option<RuntimeEnemyId>,
//     _effector: &mut Effecter,
// ) -> Result<SkillCost, WinOrLoseOrNextwave> {
//     todo!()
// }

// const SKILL: StaticSkillData = StaticSkillData {
//     id: 1,
//     call: skill_fn,
// };

// const ENEMY: StaticEnemyData = StaticEnemyData {
//     id: 1,
//     name: "aa",
//     potential: Potential::new(10.0, 10.0, 10.0, 10.0, 10.0),
//     select_skill_fn,
// };
// fn select_skill_fn(_enemy_id: RuntimeEnemyId, _state: &GameState) -> &'static EnemySkill {
//     &ENEMY_SKILL
// }
// const ENEMY_SKILL: EnemySkill = EnemySkill {
//     id: 1,
//     call: enemy_skill_fn,
// };

// fn enemy_skill_fn(
//     _enemy_id: RuntimeEnemyId,
//     _effector: &mut Effecter,
// ) -> Result<(), WinOrLoseOrNextwave> {
//     todo!()
// }

// fn args() -> ButtleArgs {
//     ButtleArgs {
//         chars: vec![CharArg {
//             level: 1,
//             static_data: &CHAR,
//             skills: vec![&SKILL],
//         }],
//         enemys: Arc::new(vec![vec![EnemyArg {
//             level: 1,
//             static_data: &ENEMY,
//         }]]),
//     }
// }

// #[test]
// fn test1() {
//     let (tx, rx) = chunnel();
//     let (mut core, mut recv) = game_core5::new_game_core(tx, rx, args()).unwrap();

//     core.send(game_core5::GameCoreActorCommand::GameStart)
//         .unwrap();

//     let res = recv.forword().unwrap().unwrap();
//     let CoreOutput::SameTime(frames) = res else {
//         panic!()
//     };
//     assert!(frames.is_empty());
//     assert_eq!(recv.state().player_mp(), TURN_START_HEAL_MP_NUM);
// }

// mod chunnel {
//     use std::error::Error;
//     use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};

//     use game_core5::{Message, MessageReceiver, MessageSender};

//     pub struct MpscMessageSender {
//         tx: Sender<Message>,
//     }

//     pub struct MpscMessageReceiver {
//         rx: Receiver<Message>,
//     }

//     impl MessageSender for MpscMessageSender {
//         fn send(&mut self, output: Message) -> Result<(), Box<dyn Error>> {
//             self.tx.send(output)?;
//             Ok(())
//         }
//     }

//     impl MessageReceiver for MpscMessageReceiver {
//         fn unblock_recv(&mut self) -> Result<Option<Message>, Box<dyn Error>> {
//             match self.rx.try_recv() {
//                 Ok(msg) => Ok(Some(msg)),
//                 Err(TryRecvError::Empty) => Ok(None),
//                 Err(e) => Err(Box::new(e)),
//             }
//         }
//     }

//     /// 外部に公開する唯一の関数
//     /// mpsc の sender/receiver を持つ実装を返す
//     pub fn chunnel() -> (impl MessageSender, impl MessageReceiver) {
//         let (tx, rx) = mpsc::channel();
//         (MpscMessageSender { tx }, MpscMessageReceiver { rx })
//     }
// }
