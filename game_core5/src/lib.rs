use std::{any::TypeId, sync::Arc};

use crate::{
    effector::Effecter,
    game_core_output_receiver::GameCoreOutputReceiver,
    living_thing::Potential,
    skill::{RuntimeSkillId, StaticSkillData},
    state::{GameState, RuntimeCharId, RuntimeEnemyId, UpdateStateMessage},
};

mod damage;
pub mod effector;
mod flow;
pub mod game_core_output_receiver;
pub mod living_thing;
pub mod passive;
pub mod skill;
pub mod state;
pub mod output;

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
// MEMO: 絶対こんなにサイズいらない
pub type StaticPassiveId = TypeId;
pub type StaticCharId = u32;
pub type StaticEnemyId = u32;
// pub type AnimationId = u32;

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
        let mut effector = Effecter::new(&mut self.state);
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

        Ok(())
    }
}

//--------------------------------------------------//
//                                                  //
//                      OUTPUT                      //
//                                                  //
//--------------------------------------------------//
pub enum CoreOutput {
    AnimatableFrames(AnimatableFrames),
    SameTime(Vec<OutputFrame>),
    GoNextWave,
    Win,
    Lose,
}

pub enum AnimationId {
    CharSkill(StaticSkillId),
    EnemySkill(StaticEnemySkillId),
}

pub struct AnimatableFrames {
    pub animation_id: AnimationId,
    pub frames: Vec<OutputFrame>,
}

pub struct OutputFrame {
    // 副作用が存在するが、作用自体は描画できない内部的な効果である場合、None
    main_effect: Option<OutputEffect>,
    sub_effects: Vec<OutputEffect>,
}
impl OutputFrame {
    pub fn main_effect(&self) -> Option<&OutputEffect> {
        self.main_effect.as_ref()
    }
    pub fn sub_effects(&self) -> &Vec<OutputEffect> {
        &self.sub_effects
    }
}
impl TryFrom<&Frame> for OutputFrame {
    // 特に伝えることはない
    // Optionでいいならそうする
    type Error = ();
    fn try_from(value: &Frame) -> Result<Self, Self::Error> {
        let main_effect = OutputEffect::try_from(&value.main_effect).ok();
        let sub_effects = value
            .sub_effects
            .iter()
            .filter_map(|e| OutputEffect::try_from(e).ok())
            .collect::<Vec<_>>();

        if main_effect.is_some() || !sub_effects.is_empty() {
            Ok(Self {
                main_effect,
                sub_effects,
            })
        } else {
            Err(())
        }
    }
}

pub enum OutputEffect {
    Damage,
    Heal,
}

impl TryFrom<&UpdateStateMessage> for OutputEffect {
    type Error = ();
    fn try_from(value: &UpdateStateMessage) -> Result<Self, Self::Error> {
        match value {
            UpdateStateMessage::Damage(_) => Ok(Self::Damage),
            _ => Err(()),
        }
    }
}
//--------------------------------------------------//
//                                                  //
//                     MESSAGE                      //
//                                                  //
//--------------------------------------------------//
#[derive(Debug, Clone)]
pub struct Message {
    inner: PrivateMessage,
}

// TurnStartなどの情報も加えなければいけない
#[derive(Debug, Clone)]
enum PrivateMessage {
    EnemySkillBegin(StaticEnemySkillId),
    EnemySkillEnd,
    SkillBegin(StaticSkillId),
    SkillEnd,
    SameTimeBegin,
    SameTimeEnd,
    Frame(Frame),
}

#[derive(Debug, Clone)]
struct Frame {
    main_effect: UpdateStateMessage,
    sub_effects: Vec<UpdateStateMessage>,
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
pub type EnemyActionFn = fn(RuntimeEnemyId, &mut Effecter) -> Result<(), WinOrLoseOrNextwave>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("すでにゲームが開始されています")]
    AlreadyGameStart,

    #[error("すでにゲームは終了しています")]
    AlreadyGameEnd,

    #[error("保持していないキャラIDです: got_id={0}")]
    NotFoundChar(StaticCharId),

    #[error("wave数が0です")]
    WavesIsEmpty,

    #[error("wave内の敵の数が不正です: got={0:?}")]
    InvalidNumEnemysInWave(Arc<Vec<Vec<EnemyArg>>>),

    #[error("使用できないスキルを使用しようとしています")]
    UnUseableSkill,

    #[error("チームメンバーの数が不正な値です: メンバー数={}", got_num_members)]
    InvalidNumTeamMembers { got_num_members: usize },

    #[error("習得スキル数が不正です")]
    InvalidNumLearnSkills(usize),

    #[error("チーム内に同じキャラクターがいます: id={0}")]
    ConfrictChar(StaticCharId),
}

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
