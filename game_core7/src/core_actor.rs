use std::collections::VecDeque;

use crate::{UserInput, effect::Effect, output::CoreOutput, state::GameState};

pub trait OutputBuffer {
    fn push_output(&mut self, output: CoreOutput);
}

pub trait RandGen {
    /// 0以上1未満の数字を出力する
    fn rnd(&mut self) -> f32;
}

pub struct CoreActor<R: RandGen> {
    rnd: R,
    state: GameState,
    effects_buffer: VecDeque<Effect>,
}

impl<R: RandGen> CoreActor<R> {
    pub fn forword(&mut self, input: UserInput, output_buffer: &mut impl OutputBuffer) {
        assert!(self.effects_buffer.is_empty());

        user_input_effect(input, &mut self.effects_buffer);
        self.state.frame(&mut self.effects_buffer);

        while let Some(effect) = self.effects_buffer.pop_front() {
            self.state.accept(&effect);
            sub_effects(&effect, &self.state, &mut self.effects_buffer);
            if let Ok(output) = CoreOutput::try_from(effect) {
                output_buffer.push_output(output);
            }
        }
    }
}

fn user_input_effect(input: UserInput, effects_buffer: &mut VecDeque<Effect>) {
    todo!()
}

fn sub_effects(main_effect: &Effect, state: &GameState, effects_buffer: &mut VecDeque<Effect>) {
    todo!()
}
