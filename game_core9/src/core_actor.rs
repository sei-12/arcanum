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

#[derive(Debug)]
pub struct GameCoreActor {
    state: GameState,
    effects_buffer: EffectsBuffer,
    game_ended: bool,
}

impl GameCoreActor {
    pub fn new(args: GameStateArgs) -> Result<Self, crate::Error> {
        Ok(Self {
            state: GameState::new(args)?,
            effects_buffer: EffectsBuffer(VecDeque::new()),
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
    effects_buffer: &mut EffectsBuffer,
) -> Result<(), crate::Error> {
    match input {
        UserInput::UseSkill { skill_id } => {
            let skill = state.get_skill(skill_id);
            if !skill.useable(state) {
                return Err(crate::Error::InvalidArgument("Unusable Skill".to_string()));
            }

            let cost = skill.skill_box().cost(skill_id, state);

            effects_buffer.push(Effect::UseSkill { skill_id });

            effects_buffer.push(Effect::AddHate {
                target_id: skill_id.owner_id(),
                num: cost.hate(),
            });

            effects_buffer.push(Effect::AddSkillCooldown {
                skill_id,
                num: cost.cooldown(),
            });

            effects_buffer.push(Effect::ConsumeMp {
                target_id: skill_id.owner_id().into(),
                num: cost.need_mp(),
            });
        }
        UserInput::None => {}
    };

    Ok(())
}

fn sub_effects(effect: &Effect, state: &GameState, effects_buffer: &mut EffectsBuffer) {
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

#[derive(Debug, Clone)]
pub struct EffectsBuffer(VecDeque<Effect>);
impl EffectsBuffer {
    pub fn push(&mut self, effect: Effect) {
        self.0.push_back(effect);
    }
    fn pop_front(&mut self) -> Option<Effect> {
        self.0.pop_front()
    }
}
