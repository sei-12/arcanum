use std::collections::VecDeque;

use crate::{
    damage::Damage,
    effect::Effect,
    game_state::{GameState, GameStateArgs, WinOrLose},
    runtime_id::RuntimeSkillId,
};

pub enum UserInput {
    UseSkill { skill_id: RuntimeSkillId },
    None,
}

pub struct GameCoreActor {
    state: GameState,
    effects_buffer: VecDeque<Effect>,
    game_ended: bool,
}

impl GameCoreActor {
    pub fn new(args: GameStateArgs) -> Result<Self, crate::Error> {
        Ok(Self {
            state: GameState::new(args)?,
            effects_buffer: VecDeque::new(),
            game_ended: false,
        })
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn tick(
        &mut self,
        input: UserInput,
        output_buffer: &mut Vec<GameCoreOutput>,
    ) -> Result<(), crate::Error> {
        if self.game_ended {
            return Err(crate::Error::AlreadyGameEnded);
        }

        user_input_effect(input, &self.state, &mut self.effects_buffer)?;

        self.state.tick(&mut self.effects_buffer);

        while let Some(effect) = self.effects_buffer.pop_front() {
            self.state.accept_effect(&effect);
            sub_effects(&effect, &self.state, &mut self.effects_buffer);
            if let Some(output_effect) = OutputEffect::try_from_effect(effect) {
                output_buffer.push(GameCoreOutput::Effect(output_effect));
            }
        }

        if let Some(win_or_lose) = self.state.check_win_or_lose() {
            self.game_ended = true;
            output_buffer.push(GameCoreOutput::Event(win_or_lose.into()));
        };

        Ok(())
    }
}

fn user_input_effect(
    input: UserInput,
    state: &GameState,
    effects_buffer: &mut VecDeque<Effect>,
) -> Result<(), crate::Error> {
    match input {
        UserInput::UseSkill { skill_id } => {
            if !state.get_skill(skill_id).useable(state) {
                return Err(crate::Error::InvalidArgument("Unusable Skill".to_string()));
            }

            effects_buffer.push_back(Effect::UseSkill { skill_id });
        }
        UserInput::None => {}
    };

    Ok(())
}

fn sub_effects(effect: &Effect, state: &GameState, effects_buffer: &mut VecDeque<Effect>) {
    if let Effect::Damage(damage) = effect {
        state.get_lt(damage.target()).passive.trigger_recv_damage(
            damage.target(),
            damage,
            state,
            effects_buffer,
        );
    }
}

pub enum GameCoreOutput {
    Effect(OutputEffect),
    Event(OutputEvent),
}

pub enum OutputEvent {
    Win,
    Lose,
}
impl From<WinOrLose> for OutputEvent {
    fn from(value: WinOrLose) -> Self {
        match value {
            WinOrLose::Win => OutputEvent::Win,
            WinOrLose::Lose => OutputEvent::Lose,
        }
    }
}

pub struct OutputEffect {
    pub kind: OutputEffectKind,
}
impl OutputEffect {
    fn try_from_effect(effect: Effect) -> Option<Self> {
        match effect {
            Effect::Damage(dmg) => Some(OutputEffect {
                kind: OutputEffectKind::Damage(dmg),
            }),
            _ => None,
        }
    }
}

pub enum OutputEffectKind {
    Damage(Damage),
}
