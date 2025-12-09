use std::collections::VecDeque;

use crate::{
    OutputBuffer,
    output::{Event, GameCoreOutput},
    receiver_side::ReceiverSide,
    runtime_id::{RuntimeCharId, RuntimeEnemyId, RuntimeSkillId},
    sender_side::SenderSide,
    state::{CharData, DungeonData, GameState},
};

#[derive(Debug, Clone)]
pub enum GameCoreActorCommand {
    UseSkill {
        user_id: RuntimeCharId,
        skill_id: RuntimeSkillId,
        target_id: Option<RuntimeEnemyId>,
    },
    TurnEnd,
    GameStart,
}

#[derive(Debug)]
pub struct GameCoreActor {
    sender_side: SenderSide,
    output_bufffer: VecDeque<GameCoreOutput>,
    receiver_side: ReceiverSide,
}

impl GameCoreActor {
    pub fn new(chars: Vec<CharData>, dungeon_data: DungeonData) -> Result<Self, crate::Error> {
        let sender_side = SenderSide::new(chars, dungeon_data)?;
        Ok(Self {
            output_bufffer: VecDeque::new(),
            receiver_side: ReceiverSide::new(sender_side.state().clone()),
            sender_side,
        })
    }

    pub fn send_cmd(&mut self, cmd: GameCoreActorCommand) {
        let res = match cmd {
            GameCoreActorCommand::GameStart => {
                self.sender_side.game_start(&mut self.output_bufffer)
            }
            GameCoreActorCommand::TurnEnd => self.sender_side.trun_end(&mut self.output_bufffer),
            GameCoreActorCommand::UseSkill {
                user_id,
                skill_id,
                target_id,
            } => self
                .sender_side
                .use_skill(user_id, target_id, skill_id, &mut self.output_bufffer),
        };

        if let Err(wln) = res {
            match wln {
                crate::WinOrLoseOrNextwave::Lose => {
                    self.output_bufffer.push(GameCoreOutput::Event(Event::Lose));
                    return;
                }
                crate::WinOrLoseOrNextwave::Win => {
                    self.output_bufffer.push(GameCoreOutput::Event(Event::Win));
                    return;
                }
                crate::WinOrLoseOrNextwave::GoNextwave => {
                    self.output_bufffer
                        .push(GameCoreOutput::Event(Event::GoNextWave));
                }
            }
        }

        self.output_bufffer.push(GameCoreOutput::WaitInput);
    }

    pub fn forward(&mut self) -> Option<GameCoreOutput> {
        self.receiver_side.forward(&mut self.output_bufffer)
    }

    pub fn state(&self) -> &GameState {
        self.receiver_side.state()
    }
}
