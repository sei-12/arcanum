use crate::game_core_output_receiver::GameCoreOutputReceiver;

pub mod passive;
pub mod game_core_output_receiver;
pub mod state;

pub type MpNum = u32;
pub type SpNum = u32;
pub type LevelNum = u32;
pub type StatusNum = f32;
pub type TurnNum = u8;
pub type CooldownNum = u32;
pub type HateNum = u32;
pub type WaveNum = u8;
pub type SkillId = u32;
pub type StaticPassiveId = u32;
pub type AnimationId = u32;

//--------------------------------------------------//
//                                                  //
//                 GAME CORE ACTOR                  //
//                                                  //
//--------------------------------------------------//

pub struct GameCoreActorCommand {}

pub struct GameCoreArgment {}

pub fn new_game_core<S, R, A>(
    sender: S,
    receiver: R,
    assets_server: A,
) -> (GameCoreActor<S, A>, GameCoreOutputReceiver<R>)
where
    S: MessegeSender,
    R: MessageReceiver,
    A: AssetsServer,
{
    todo!()
    /*     let core_actor = GameCoreActor {
        sender,
        assets_server,
    };

    let core_receiver = GameCoreOutputReceiver {
        receiver,
        assets_server,
    };

    (core_actor, core_receiver) */
}

pub struct GameCoreActor<S: MessegeSender, A: AssetsServer> {
    assets_server: A,
    sender: S,
}

impl<S: MessegeSender, A: AssetsServer> GameCoreActor<S, A> {
    pub fn send(&mut self, command: GameCoreActorCommand) {}
}

//--------------------------------------------------//
//                                                  //
//                  ASSETS SERVER                   //
//                                                  //
//--------------------------------------------------//
/// 関数郡のようなもの
/// 実態は軽量でコピー可能であるべき
pub trait AssetsServer: Copy {}

//--------------------------------------------------//
//                                                  //
//                      OUTPUT                      //
//                                                  //
//--------------------------------------------------//
pub enum Output {
    AnimatableFrames(AnimatableFrames),
    SameTime(Vec<OutputFrame>),
    GoNextWave,
    Win,
    Lose,
}

pub struct AnimatableFrames {
    animation_id: AnimationId,
    frames: Vec<OutputFrame>,
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
            UpdateStateMessage::Damage => Ok(Self::Damage),
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

#[derive(Debug, Clone)]
enum PrivateMessage {
    SkillBegin(SkillId),
    SkillEnd,
    SameTimeBegin,
    SameTimeEnd,
    Frame(Frame),
}

impl PrivateMessage {
    pub(crate) fn is_begin(&self) -> bool {
        matches!(
            self,
            PrivateMessage::SameTimeBegin | PrivateMessage::SkillBegin(_)
        )
    }

    pub(crate) fn is_end(&self) -> bool {
        matches!(self, Self::SameTimeEnd | Self::SkillEnd)
    }
}

#[derive(Debug, Clone)]
struct Frame {
    main_effect: UpdateStateMessage,
    sub_effects: Vec<UpdateStateMessage>,
}

#[derive(Debug, Clone)]
enum UpdateStateMessage {
    Damage,
    HealMp,
}

//--------------------------------------------------//
//                                                  //
//                      STATE                       //
//                                                  //
//--------------------------------------------------//

pub enum WinOrLoseOrNextwave {
    Win,
    Lose,
    Nextwave,
}


//--------------------------------------------------//
//                                                  //
//             MESSAGE SENDER RECEIVER              //
//                                                  //
//--------------------------------------------------//
pub trait MessegeSender {
    fn send(&mut self, output: Message) -> Result<(), Box<dyn std::error::Error>>;
}
pub trait MessageReceiver {
    fn unblock_recv(&mut self) -> Result<Option<Message>, Box<dyn std::error::Error>>;
}
