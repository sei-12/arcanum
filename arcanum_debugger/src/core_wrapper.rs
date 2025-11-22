use std::{cell::RefCell, collections::VecDeque};

use game_core::{
    chars::StaticCharId,
    container_args::ContainerArgs,
    error::GameError,
    game_core_actor::{GameCoreActor, GameCoreActorCommand},
    game_state::GameState,
    screen_actor::ScreenActorSender,
    skills::StaticSkillId,
};
use rand::{Rng, rng};

#[derive(Debug)]
pub struct CoreWrapper {
    core: GameCoreActor,
    command_que: RefCell<VecDeque<GameCoreActorCommand>>,
}

impl CoreWrapper {
    pub fn new(args: &ContainerArgs, screen: Box<dyn ScreenActorSender>) -> Self {
        let core_actor = GameCoreActor::new(args, screen).unwrap();
        Self {
            core: core_actor,
            command_que: RefCell::new(VecDeque::new()),
        }
    }

    pub fn update(&mut self) -> Result<(), GameError> {
        let mut borrowed_que = self.command_que.borrow_mut();
        while let Some(cmd) = borrowed_que.pop_front() {
            self.core.send(cmd)?;
        }
        Ok(())
    }
    pub fn get_state(&self) -> &GameState {
        self.core.get_state()
    }

    pub fn use_skill(&self, static_user_id: StaticCharId, static_skill_id: StaticSkillId) {
        let cmd = GameCoreActorCommand::UseSkill {
            static_user_id,
            static_skill_id,
        };

        self.command_que.borrow_mut().push_back(cmd);
    }

    pub fn start_game(&self) {
        let random_seed: u64 = {
            let mut rng = rng();
            rng.random()
        };

        self.command_que
            .borrow_mut()
            .push_back(GameCoreActorCommand::GameStart { random_seed });
    }

    pub fn turn_end(&self) {
        self.command_que
            .borrow_mut()
            .push_back(GameCoreActorCommand::PlayerTrunEnd);
    }
}
