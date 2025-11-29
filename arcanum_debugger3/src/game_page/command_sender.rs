use std::{cell::RefCell, sync::mpsc};

use game_core3::{
    args::ContainerArgs,
    enemys::RuntimeEnemyId,
    game_core::{GameCoreActor, command::GameCoreActorCommand},
    screen_actor::ScreenActorSender,
    skill::StaticSkillId,
    state::GameState,
    static_char::StaticCharId,
};

#[derive(Debug)]
struct Sender {
    sender: mpsc::Sender<game_core3::event::Event>,
}
impl ScreenActorSender for Sender {
    fn send(&mut self, event: game_core3::event::Event) {
        self.sender.send(event).unwrap();
    }
}

#[derive(Debug)]
pub struct CommandSender {
    // commands: RefCell<VecDeque<GameCoreActorCommand>>,
    core: RefCell<GameCoreActor<Sender>>,
}

impl CommandSender {
    pub fn new(
        arg: &ContainerArgs,
    ) -> Result<(Self, mpsc::Receiver<game_core3::event::Event>, GameState), game_core3::Error>
    {
        let (tx, rx) = mpsc::channel();
        let core = GameCoreActor::new(arg, Sender { sender: tx })?;
        let state = core.get_state().clone();
        Ok((
            Self {
                core: RefCell::new(core), // commands: RefCell::new(VecDeque::new()),
            },
            rx,
            state,
        ))
    }

    pub fn game_start(&self) -> Result<(), game_core3::Error> {
        let mut core_mut = self.core.borrow_mut();
        core_mut.accept(GameCoreActorCommand::GameStart)
    }

    pub fn use_skill(
        &self,
        static_char_id: StaticCharId,
        skill_id: StaticSkillId,
    ) -> Result<(), game_core3::Error> {
        let mut core_mut = self.core.borrow_mut();
        core_mut.accept(GameCoreActorCommand::UseSkill {
            user: static_char_id,
            skill: skill_id,
        })
    }

    pub fn turnend(&self) -> Result<(), game_core3::Error> {
        let mut core_mut = self.core.borrow_mut();
        core_mut.accept(GameCoreActorCommand::TurnEnd)
    }

    pub fn change_focus(&self, enemy_id: RuntimeEnemyId) -> Result<(), game_core3::Error> {
        self.core
            .borrow_mut()
            .accept(GameCoreActorCommand::ChangeFocusEnemy { enemy_id })
    }
}
