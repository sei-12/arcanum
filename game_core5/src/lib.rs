use std::{any::TypeId, sync::Arc};

use crate::{
    effector::EffectAccepter,
    game_core_output_receiver::{CoreMessage, GameCoreOutputReceiver},
    living_thing::Potential,
    skill::{RuntimeSkillId, StaticSkillData},
    state::{GameState, RuntimeCharId, RuntimeEnemyId},
};

pub mod effect;

mod damage;
pub mod effector;
pub mod error;
mod flow;
pub mod game_core_output_receiver;
pub mod living_thing;
pub mod passive;
pub mod skill;
pub mod state;

pub use crate::error::Error;

pub const NUM_MAX_CHAR_IN_TEAM: u8 = 4;
pub const NUM_MAX_LEARN_SKILLS: usize = 6;
pub const NUM_MAX_ENEMYS_IN_WAVE: usize = 5;
pub const TURN_START_HEAL_MP_NUM: MpNum = 100;
pub const SKILL_COOLDOWN_HEAL_BASE: CooldownNum = 50;
pub const TURN_START_HEAL_SP_NUM: SpNum = 50;

pub type MpNum = u32;
pub type SpNum = u32;
pub type LevelNum = u32;
pub type StatusNum = f32;
pub type TurnNum = u8;
pub type CooldownNum = u32;
pub type HateNum = u32;
pub type WaveNum = u8;
pub type StaticEnemySkillId = u32;
pub type StaticSkillId = u32;
pub type StaticPassiveId = TypeId;
pub type StaticCharId = u32;
pub type StaticEnemyId = u32;

//--------------------------------------------------//
//                                                  //
//                 GAME CORE ACTOR                  //
//                                                  //
//--------------------------------------------------//

pub enum GameCoreActorCommand {
    UseSkill {
        user_id: RuntimeCharId,
        skill_id: RuntimeSkillId,
        focused_enemy_id: Option<RuntimeEnemyId>,
    },
    TurnEnd,
    GameStart,
}

pub struct GameCoreArgment {}

pub fn new_game_core<S, R>(
    sender: S,
    receiver: R,
    buttle_args: ButtleArgs,
) -> Result<(GameCoreActor<S>, GameCoreOutputReceiver<R>), crate::Error>
where
    S: MessageSender,
    R: MessageReceiver,
{
    let core_actor = GameCoreActor::new(sender, buttle_args)?;
    let receiver = GameCoreOutputReceiver::new(receiver, core_actor.state.clone());
    Ok((core_actor, receiver))
}

pub struct GameCoreActor<S: MessageSender> {
    sender: S,
    state: GameState,
}

impl<S: MessageSender> GameCoreActor<S> {
    fn new(sender: S, args: ButtleArgs) -> Result<Self, crate::Error> {
        Ok(Self {
            sender,
            state: GameState::new(&args)?,
        })
    }

    pub fn send(
        &mut self,
        command: GameCoreActorCommand,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
/*         let mut effector = Effecter::new(&mut self.state);
        let _ = match command {
            GameCoreActorCommand::GameStart => flow::game_start(&mut effector),
            GameCoreActorCommand::TurnEnd => flow::turn_end(&mut effector),
            GameCoreActorCommand::UseSkill {
                user_id,
                skill_id,
                focused_enemy_id,
            } => flow::use_skill(&mut effector, user_id, skill_id, focused_enemy_id),
        };

        for msg in effector.take_messages().into_iter() {
            self.sender.send(Message { inner: msg })?;
        }

        Ok(()) */
    }
}
//--------------------------------------------------//
//                                                  //
//                     MESSAGE                      //
//                                                  //
//--------------------------------------------------//
#[derive(Debug, Clone)]
pub struct Message {
    inner: CoreMessage,
}

//--------------------------------------------------//
//                                                  //
//                      STATE                       //
//                                                  //
//--------------------------------------------------//

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WinOrLoseOrNextwave {
    Win,
    Lose,
    Nextwave,
}

impl WinOrLoseOrNextwave {
    fn is_win_or_lose(&self) -> bool {
        *self == WinOrLoseOrNextwave::Win || *self == WinOrLoseOrNextwave::Lose
    }
}

//--------------------------------------------------//
//                                                  //
//             MESSAGE SENDER RECEIVER              //
//                                                  //
//--------------------------------------------------//
pub trait MessageSender {
    fn send(&mut self, output: Message) -> Result<(), Box<dyn std::error::Error>>;
}
pub trait MessageReceiver {
    fn unblock_recv(&mut self) -> Result<Option<Message>, Box<dyn std::error::Error>>;
}

//--------------------------------------------------//
//                                                  //
//                 STATIC CHAR DATA                 //
//                                                  //
//--------------------------------------------------//
#[derive(Debug)]
pub struct StaticCharData {
    pub id: StaticCharId,
    pub name: &'static str,
    pub potential: Potential,
}

#[derive(Debug, Clone)]
pub struct StaticEnemyData {
    pub id: StaticEnemyId,
    pub name: &'static str,
    pub potential: Potential,
    pub select_skill_fn: SelectEnemySkillFn,
}

pub struct EnemySkill {
    pub id: StaticEnemySkillId,
    pub call: EnemyActionFn,
}

pub type SelectEnemySkillFn = fn(RuntimeEnemyId, &GameState) -> &'static EnemySkill;
pub type EnemyActionFn =
    fn(RuntimeEnemyId, &mut dyn EffectAccepter) -> Result<(), WinOrLoseOrNextwave>;

pub struct ButtleArgs {
    pub chars: Vec<CharArg>,
    pub enemys: Arc<Vec<Vec<EnemyArg>>>,
}

pub struct CharArg {
    pub level: LevelNum,
    pub static_data: &'static StaticCharData,
    pub skills: Vec<&'static StaticSkillData>,
}

#[derive(Debug)]
pub struct EnemyArg {
    pub level: LevelNum,
    pub static_data: &'static StaticEnemyData,
}
