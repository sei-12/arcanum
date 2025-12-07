use std::collections::VecDeque;

use crate::{
    OutputBuffer,
    output::GameCoreOutput,
    receiver_side::ReceiverSide,
    runtime_id::{RuntimeCharId, RuntimeSkillId},
    sender_side::SenderSide,
    state::GameState,
};

pub enum GaemCoreActorCommand {
    UseSkill {
        user_id: RuntimeCharId,
        skill_id: RuntimeSkillId,
    },
    TurnEnd,
    GameStart,
}

pub struct GameCoreActor {
    sender_side: SenderSide,
    output_bufffer: VecDeque<GameCoreOutput>,
    receiver_side: ReceiverSide,
}

impl GameCoreActor {
    pub fn send_cmd(&mut self, cmd: GaemCoreActorCommand) {
        match cmd {
            GaemCoreActorCommand::GameStart => {
                let _ = self.sender_side.game_start(&mut self.output_bufffer);
            }
            GaemCoreActorCommand::TurnEnd => {
                let _ = self.sender_side.trun_end(&mut self.output_bufffer);
            }
            GaemCoreActorCommand::UseSkill { user_id, skill_id } => {
                let _ = self
                    .sender_side
                    .use_skill(user_id, skill_id, &mut self.output_bufffer);
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
