use std::collections::VecDeque;

use crate::{
    UserInput,
    effect::Effect,
    output::CoreOutput,
    skill::UsingSkillState,
    state::{GameState, GameStateArgs},
};

pub trait OutputBuffer {
    fn push_output(&mut self, output: CoreOutput);
}

pub struct CoreActor {
    state: GameState,
    effects_buffer: VecDeque<Effect>,
}

impl CoreActor {
    pub fn new(buttle_args: GameStateArgs) -> Result<Self, crate::Error> {
        Ok(Self {
            effects_buffer: VecDeque::new(),
            state: GameState::new(buttle_args)?,
        })
    }

    pub fn forword(
        &mut self,
        input: UserInput,
        output_buffer: &mut impl OutputBuffer,
    ) -> Result<(), crate::Error> {
        self.state.tick();

        assert!(self.effects_buffer.is_empty());
        let mut ctx = CtxContainer {
            effects_buffer: &mut self.effects_buffer,
        };

        user_input_effect(input, &self.state, &mut ctx)?;
        self.state.frame(&mut ctx);

        while let Some(effect) = ctx.effects_buffer.pop_front() {
            self.state.accept(&effect);
            sub_effects(&effect, &self.state, &mut ctx);
            output_buffer.push_output(CoreOutput::Effect(effect));
        }

        Ok(())
    }
}

// MEMO: 必要無くなった
pub struct CtxContainer<'a> {
    pub effects_buffer: &'a mut VecDeque<Effect>,
}

fn user_input_effect(
    input: UserInput,
    state: &GameState,
    ctx: &mut CtxContainer,
) -> Result<(), crate::Error> {
    match input {
        UserInput::UseSkill { char_id, skill_id } => {
            let char = state.try_get_char(char_id)?;
            let _ = char.try_get_skill(skill_id)?;

            if !char.can_start_skill(skill_id) {
                return Err(crate::Error::UnUseableSkill);
            }

            ctx.effects_buffer.push_back(Effect::UseSkill {
                user_id: char_id,
                skill_id,
                state: UsingSkillState::new(),
            });
        }
        UserInput::None => {}
    };

    Ok(())
}

fn sub_effects(main_effect: &Effect, state: &GameState, ctx: &mut CtxContainer) {
    if let Effect::Damage(dmg) = main_effect {
        let target = state.get_lt(dmg.target());
        target
            .passive
            .trigger_recv_dmg(dmg.target(), state, dmg, ctx);

        if let Some(causer_id) = dmg.causer().to_lt_id() {
            let causer = state.get_lt(causer_id);
            causer.passive.trigger_deal_dmg(causer_id, state, dmg, ctx);
        }
    }
}
