pub mod command;

mod event_que;

use crate::{
    TURN_START_HEAL_MP_NUM,
    container::{command::GameCoreActorCommand, event_que::EventsQue},
    event::{self, EventsQuePusher},
    screen_actor::ScreenActorSender,
    skill::{SkillTrait, StaticSkillId},
    state::GameState,
    static_char::StaticCharId,
};

pub struct GameCoreActor<S: ScreenActorSender> {
    screen_actor_sender: S,
    state: GameState,
}

impl<S: ScreenActorSender> GameCoreActor<S> {
    pub fn accept(&mut self, cmd: GameCoreActorCommand) -> Result<(), crate::Error> {
        match cmd {
            GameCoreActorCommand::UseSkill { user, skill } => {
                self.use_skill(user, skill)?;
            }
            GameCoreActorCommand::TurnEnd => {
                self.turn_end();
            }
            _ => {
                todo!()
            }
        }
        Ok(())
    }
}

//--------------------------------------------------//
//                                                  //
//                       FLOW                       //
//                                                  //
//--------------------------------------------------//

impl<S: ScreenActorSender> GameCoreActor<S> {
    fn use_skill(
        &mut self,
        user_id: StaticCharId,
        skill_id: StaticSkillId,
    ) -> Result<(), crate::Error> {
        let mut events = EventsQue::default();
        let user = self.state.chars().get_char_by_static_id(user_id)?;
        let skill = user.skills.get(skill_id)?;
        let result = skill.call(user, &self.state, &mut events);
        result.to_events(&mut events);
        self.accept_events(&mut events);
        Ok(())
    }

    fn turn_end(&mut self) {
        let mut events = EventsQue::default();
        self.enemy_turn_start(&mut events);
    }

    fn enemy_turn_start(&mut self, events: &mut EventsQue) {
        events.push(event::Event::HealMp {
            side: crate::state::Side::Enemy,
            mp: TURN_START_HEAL_MP_NUM + self.state.enemy().lt().passive.status().add_heal_mp,
        });

        self.accept_events(events);

        let enemy = self.state.enemy().lt();
        enemy
            .passive
            .trigger_turn_start(crate::state::LtId::Enemy, &self.state, events);

        self.accept_events(events);
    }

    fn accept_events(&mut self, events: &mut EventsQue) {
        while let Some(event) = events.pop() {
            self.state.accept_event(event.clone());
            self.screen_actor_sender.send(event);
        }
    }
}
