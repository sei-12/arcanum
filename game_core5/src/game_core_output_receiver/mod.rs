mod effects;

use crate::{
    CoreOutput, Frame, MessageReceiver, OutputFrame, PrivateMessage, StaticEnemySkillId,
    StaticSkillId, WinOrLoseOrNextwave, state::GameState,
};

//--------------------------------------------------//
//                                                  //
//            GAME CORE OUTPUT RECEIVER             //
//                                                  //
//--------------------------------------------------//
pub struct GameCoreOutputReceiver<R: MessageReceiver> {
    receiver: R,
    state: GameState,
    buffer: Option<BeginedMessageBuffer>,

    // MEMO: ロジックが複雑になってきたらドキュメントレベルのルールではなく、型レベルでの制約にしてもいい
    // 現状は必要ないと判断した
    /// Rule: Win, Lose, GonextWaveのいずれかしか入らない
    output_tmp: Option<CoreOutput>,
}

impl<R: MessageReceiver> GameCoreOutputReceiver<R> {
    pub(crate) fn new(receiver: R, state: GameState) -> Self {
        Self {
            receiver,
            state,
            buffer: None,
            output_tmp: None,
        }
    }

    pub fn forword(&mut self) -> Result<Option<CoreOutput>, Box<dyn std::error::Error>> {
        if let Some(output) = self.output_tmp.take() {
            debug_assert!({
                matches!(output, CoreOutput::Win)
                    || matches!(output, CoreOutput::Lose)
                    || matches!(output, CoreOutput::GoNextWave)
            },);
            return Ok(Some(output));
        }

        let buffer = self.buffer.take().unwrap_or({
            let Some(msg) = self.receiver.unblock_recv()? else {
                return Ok(None);
            };

            let begin = match msg.inner {
                PrivateMessage::SkillBegin(skill_id) => Begin::Skill(skill_id),
                PrivateMessage::SameTimeBegin => Begin::SameTime,
                PrivateMessage::EnemySkillBegin(skill_id) => Begin::EnemySkill(skill_id),
                _ => panic!("メッセージはbeginから始まる必要がある"),
            };

            BeginedMessageBuffer::new(begin)
        });

        let msg_block = match take_messsage_block_from_receiver(&mut self.receiver, buffer)? {
            Ok(msg_block) => msg_block,
            Err(buffer) => {
                self.buffer = Some(buffer);
                return Ok(None);
            }
        };

        let (begin, frames) = match msg_block {
            MessageBlock::SameTime(frames) => (Begin::SameTime, frames),
            MessageBlock::Skill(skill_id, frames) => (Begin::Skill(skill_id), frames),
            MessageBlock::EnemySkill(skill_id, frames) => (Begin::EnemySkill(skill_id), frames),
        };

        let output_frames = frames
            .iter()
            .filter_map(|f| OutputFrame::try_from(f).ok())
            .collect();

        if let Some(result) = apply_frames_to_state(&mut self.state, &frames) {
            match result {
                WinOrLoseOrNextwave::Lose => self.output_tmp = Some(CoreOutput::Lose),
                WinOrLoseOrNextwave::Win => self.output_tmp = Some(CoreOutput::Win),
                WinOrLoseOrNextwave::Nextwave => self.output_tmp = Some(CoreOutput::GoNextWave),
            }
        }

        let output = match begin {
            Begin::EnemySkill(skill_id) => CoreOutput::AnimatableFrames(crate::AnimatableFrames {
                animation_id: crate::AnimationId::EnemySkill(skill_id),
                frames: output_frames,
            }),
            Begin::Skill(skill_id) => CoreOutput::AnimatableFrames(crate::AnimatableFrames {
                animation_id: crate::AnimationId::CharSkill(skill_id),
                frames: output_frames,
            }),
            Begin::SameTime => CoreOutput::SameTime(output_frames),
        };

        Ok(Some(output))
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }
}

enum MessageBlock {
    Skill(StaticSkillId, Vec<Frame>),
    EnemySkill(StaticEnemySkillId, Vec<Frame>),
    SameTime(Vec<Frame>),
}

#[derive(Debug)]
struct BeginedMessageBuffer {
    begin: Begin,
    frames: Vec<Frame>,
}

#[derive(Debug)]
enum Begin {
    Skill(StaticSkillId),
    EnemySkill(StaticEnemySkillId),
    SameTime,
}

impl BeginedMessageBuffer {
    fn new(begin: Begin) -> Self {
        Self {
            begin,
            frames: Vec::new(),
        }
    }

    fn push(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    fn into_same_time_message_block(self) -> MessageBlock {
        if !matches!(self.begin, Begin::SameTime) {
            panic!("beginとendが対応していない self={:?}", self);
        }
        MessageBlock::SameTime(self.frames)
    }

    fn into_skill_message_block(self) -> MessageBlock {
        let Begin::Skill(skill_id) = self.begin else {
            panic!("beginとendが対応していない self={:?}", self);
        };

        MessageBlock::Skill(skill_id, self.frames)
    }

    fn into_enemy_skill_message_block(self) -> MessageBlock {
        let Begin::EnemySkill(skill_id) = self.begin else {
            panic!("beginとendが対応していない self={:?}", self);
        };

        MessageBlock::EnemySkill(skill_id, self.frames)
    }
}

/// receiverからメッセージを取り出してバッファーに入れる
/// bufferの所有権を受け取るが、メッセージが足りずBlockの作成ができない場合にErr値として返す
fn take_messsage_block_from_receiver<R>(
    receiver: &mut R,
    mut buffer: BeginedMessageBuffer,
) -> Result<Result<MessageBlock, BeginedMessageBuffer>, Box<dyn std::error::Error>>
where
    R: MessageReceiver,
{
    while let Some(msg) = receiver.unblock_recv()? {
        match msg.inner {
            PrivateMessage::Frame(f) => {
                buffer.push(f);
            }
            PrivateMessage::SameTimeEnd => {
                return Ok(Ok(buffer.into_same_time_message_block()));
            }
            PrivateMessage::SkillEnd => {
                return Ok(Ok(buffer.into_skill_message_block()));
            }
            PrivateMessage::EnemySkillEnd => {
                return Ok(Ok(buffer.into_enemy_skill_message_block()));
            }
            PrivateMessage::SameTimeBegin
            | PrivateMessage::SkillBegin(_)
            | PrivateMessage::EnemySkillBegin(_) => panic!(
                "endが呼ばれる前にbeginが呼ばれた: buffer={:?} got={:?}",
                &buffer, msg.inner
            ),
        }
    }

    Ok(Err(buffer))
}

fn apply_frames_to_state(state: &mut GameState, frames: &[Frame]) -> Option<WinOrLoseOrNextwave> {
    let mut result = None;
    let mut effects = effects::Effects::new(frames);

    while let Some(effect) = effects.next() {
        let Some(main_effect_result) = state.update(effect) else {
            continue;
        };

        if effects.has_remaining() && !matches!(main_effect_result, WinOrLoseOrNextwave::Nextwave) {
            // GoNextWaveの場合は残りの副作用も適用し続ける
            // Win,Loseの場合はそこで中断しなければならない
            panic!()
        }

        result = Some(main_effect_result)
    }

    result
}
