pub mod command;

mod event_que;

use crate::{
     TURN_START_HEAL_MP_NUM,
    args::ContainerArgs,
    event::{self, Event, EventsQuePusher},
    game_core::{command::GameCoreActorCommand, event_que::EventsQue},
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
    pub fn new(arg: &ContainerArgs, screen_actor_sender: S) -> Result<Self, crate::Error> {
        Ok(Self {
            screen_actor_sender,
            state: GameState::new(arg)?,
        })
    }
    pub fn accept(&mut self, cmd: GameCoreActorCommand) -> Result<(), crate::Error> {
        if self.state.check_game_end().game_ended() {
            return Err(crate::Error::AlreadyGameEnd);
        }

        match cmd {
            GameCoreActorCommand::UseSkill { user, skill } => {
                self.use_skill(user, skill)?;
            }
            GameCoreActorCommand::TurnEnd => {
                self.turn_end();
            }
            GameCoreActorCommand::GameStart => {
                self.game_start();
            }
            GameCoreActorCommand::ChangeFocusEnemy { enemy_id } => {
                let mut events = EventsQue::default();
                events.push(Event::ChangeFocusEnemy { enemy_id });
                self.accept_events(&mut events);
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
        result.to_events(&mut events, user.runtime_id(), skill_id);
        self.accept_events(&mut events);
        Ok(())
    }

    fn turn_end(&mut self) {
        let mut events = EventsQue::default();
        self.enemy_turn_start(&mut events);

        if self.accept_events(&mut events) {
            return;
        };

        self.player_turn_start(&mut events);
    }

    fn game_start(&mut self) {
        let mut events = EventsQue::default();
        self.player_turn_start(&mut events);
    }

    fn enemy_turn_start(&mut self, events: &mut EventsQue) {
        events.push(event::Event::TurnStart(crate::state::Side::Enemy));
        events.push(event::Event::HealMp {
            side: crate::state::Side::Enemy,
            mp: TURN_START_HEAL_MP_NUM,
        });

        if self.accept_events(events) {
            return;
        };

        self.state.enemys().current_wave_enemys().for_each(|enemy| {
            enemy
                .lt()
                .passive
                .trigger_turn_start(enemy.lt_id(), &self.state, events);
        });

        self.accept_events(events);
    }

    fn player_turn_start(&mut self, events: &mut EventsQue) {
        events.push(event::Event::TurnStart(crate::state::Side::Player));
        events.push(event::Event::HealMp {
            side: crate::state::Side::Player,
            mp: TURN_START_HEAL_MP_NUM,
        });

        if self.accept_events(events) {
            return;
        };

        self.state.chars().chars().iter().for_each(|char| {
            char.lt()
                .passive
                .trigger_turn_start(char.lt_id(), &self.state, events);
        });

        self.accept_events(events);
    }

    /// 勝ちもしくは負けならtrue
    fn accept_events(&mut self, events: &mut EventsQue) -> bool {
        while let Some(event) = events.pop() {
            self.state.accept_event(event.clone());
            self.screen_actor_sender.send(event);

            let check = self.state.check_game_end();
            match check {
                crate::state::CheckGameEndResult::Win | crate::state::CheckGameEndResult::Lose => {
                    let result = match check {
                        crate::state::CheckGameEndResult::Lose => crate::GameResult::Lose,
                        crate::state::CheckGameEndResult::Win => crate::GameResult::Win,
                        _ => panic!(),
                    };

                    events.clear();
                    let event = event::Event::GameEnd(result);
                    self.state.accept_event(event.clone());
                    self.screen_actor_sender.send(event);
                    return true;
                }
                crate::state::CheckGameEndResult::GoNextWave => {
                    events.clear();
                    events.push(event::Event::GoNextWave);
                }
                _ => {}
            }
        }

        false
    }
}
