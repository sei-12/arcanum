use game_core3::{
    args::ContainerArgs, state::GameState,
};

use crate::game_page::{command_sender::CommandSender, event_receiver::EventReceiver};

mod event_to_log;
mod command_sender;
pub mod draw;
mod event_receiver;

pub struct GamePageState {
    sender: CommandSender,
    receiver: EventReceiver,
}

impl GamePageState {
    pub fn new(args: &ContainerArgs) -> Result<Self, game_core3::Error> {
        let (sender, recv, state) = CommandSender::new(args)?;
        let receiver = EventReceiver::new(recv, state);
        Ok(Self { sender, receiver })
    }

    pub fn game_state(&self) -> &GameState {
        self.receiver.state()
    }
    pub fn logs(&self) -> &Vec<String> {
        self.receiver.log()
    }
    pub fn sender(&self) -> &CommandSender {
        &self.sender
    }
}
