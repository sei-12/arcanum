use crate::{
    any_message::AnyMessageBox,
    core_actor::EffectsBuffer,
    effect::Effect,
    game_state::GameState,
    progress_state::ProgressState,
    runtime_id::RuntimeSkillId,
    skill::{CharSkillProgress, CharSkillProgressKind},
    skill_impl_utils::skill_chunks::SkillChunks,
};
use std::{fmt::Debug, sync::Arc};

pub type EffectsFn = dyn Fn(RuntimeSkillId, &GameState, &mut EffectsBuffer);

mod skill_chunks;

#[derive(Clone)]
pub struct SkillEffectUnit {
    time_ms: u16,
    progress_kind: CharSkillProgressKind,
    effects: Arc<EffectsFn>,
}
impl Debug for SkillEffectUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "SkillEffectUnit {{ time_ms: {:?}, progress_kind: {:?} }}",
            self.time_ms, self.progress_kind
        ))
    }
}

impl SkillEffectUnit {
    pub fn new(
        time_ms: u16,
        progress_kind: CharSkillProgressKind,
        effects: impl Fn(RuntimeSkillId, &GameState, &mut EffectsBuffer) + 'static,
    ) -> Result<Self, SkillEffectUnitError> {
        Self::new_with_arc(time_ms, progress_kind, Arc::new(effects))
    }

    pub fn new_with_arc(
        time_ms: u16,
        progress_kind: CharSkillProgressKind,
        effects: Arc<EffectsFn>,
    ) -> Result<SkillEffectUnit, SkillEffectUnitError> {
        if time_ms == 0 {
            return Err(SkillEffectUnitError::TimeIsZero);
        }

        Ok(Self {
            time_ms,
            progress_kind,
            effects,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillEffectUnitError {
    TimeIsZero,
}

// MEMO: Cloneのコストが気になるなら times, units, chunks をArcで囲ってもいい
#[derive(Debug, Clone)]
pub struct SkillEffectUnits {
    progress: f32,
    step: usize,
    startd: bool,
    times: Vec<f32>,
    units: Vec<SkillEffectUnit>,
    chunks: SkillChunks,
}

impl SkillEffectUnits {
    pub fn new(units: Vec<SkillEffectUnit>) -> Result<Self, SkillEffectUnitsError> {
        if units.is_empty() {
            return Err(SkillEffectUnitsError::InvalidArg);
        }

        let mut times = Vec::with_capacity(units.len());
        times.push(units.first().unwrap().time_ms as f32);

        for v in units.iter().skip(1) {
            times.push(times.last().unwrap() + v.time_ms as f32);
        }

        assert_eq!(times.len(), units.len());

        Ok(Self {
            chunks: SkillChunks::new(&units),
            units,
            times,
            progress: 0.0,
            startd: false,
            step: 0,
        })
    }

    pub fn tick(
        &self,
        id: RuntimeSkillId,
        state: &GameState,
        effects_buffer: &mut EffectsBuffer,
    ) -> Result<(), SkillEffectUnitsError> {
        let (effect_fucntions, msg, ended) =
            self.tick_inner(state.get_char(id.char_id).lt().speed())?;

        for effect_fn in effect_fucntions {
            effect_fn.as_ref()(id, state, effects_buffer);
        }

        effects_buffer.push(Effect::UpdateSkillState { skill_id: id, msg });

        if ended {
            effects_buffer.push(Effect::EndSkill { skill_id: id });
        }

        Ok(())
    }

    // iterで返してもいいけど、面倒なのでやめとく
    fn tick_inner(
        &self,
        add_progress: f32,
    ) -> Result<(Vec<Arc<EffectsFn>>, AnyMessageBox, bool), SkillEffectUnitsError> {
        assert_eq!(self.times.len(), self.units.len());
        debug_assert!(self.times.iter().is_sorted());
        debug_assert!({
            // stepsが要素数以上ということは進捗は100%以上である必要があるし
            // stepsが要素数未満ということは進捗は100%未満である必要がある。
            let check_steps = self.step >= self.times.len();
            let check_progress = *self.times.last().unwrap() <= self.progress;
            (check_steps && check_progress) || (!check_steps && !check_progress)
        });

        if !self.startd {
            return Err(SkillEffectUnitsError::NotStarted);
        }

        let mut current_step = self.step;
        let mut effect_functions = Vec::new();
        let current_progress = self.progress + add_progress;

        while self
            .times
            .get(current_step)
            .is_some_and(|t| *t <= current_progress)
        {
            effect_functions.push(self.units[current_step].effects.clone());
            current_step += 1;
        }

        let ended = current_step >= self.times.len();

        Ok((
            effect_functions,
            AnyMessageBox::new(UpdateUnits {
                add_progress,
                steps: current_step,
            }),
            ended,
        ))
    }

    pub fn update<'a>(&mut self, msg: &'a AnyMessageBox) -> Option<&'a AnyMessageBox> {
        let Some(own_msg) = msg.downcast_ref::<UpdateUnits>() else {
            return Some(msg);
        };

        self.progress += own_msg.add_progress;
        self.step = own_msg.steps;

        None
    }

    pub fn start(&mut self) -> Result<(), SkillEffectUnitsError> {
        if self.startd {
            return Err(SkillEffectUnitsError::AlreadyStared);
        }

        self.progress = 0.0;
        self.step = 0;
        self.startd = true;

        Ok(())
    }

    pub fn end(&mut self) -> Result<(), SkillEffectUnitsError> {
        if !self.startd {
            return Err(SkillEffectUnitsError::NotStarted);
        }

        self.startd = false;
        self.progress = 0.0;
        self.step = 0;

        Ok(())
    }

    /// 開始済みではない場合noneを返す
    pub fn current_progress(&self) -> Option<CharSkillProgress> {
        assert!(!self.times.is_empty());

        if !self.startd {
            return None;
        }

        let overall_progress =
            ProgressState::new(self.progress, *self.times.last().unwrap()).unwrap();

        let (kind, chunk_progress) = self.chunks.current_chunk_progress(self.progress);

        Some(CharSkillProgress {
            chunk_progress,
            kind,
            overall_progress,
        })
    }
}

#[derive(Debug, Clone)]
struct UpdateUnits {
    steps: usize,
    add_progress: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillEffectUnitsError {
    NotStarted,
    AlreadyStared,
    InvalidArg,
}

#[cfg(test)]
mod tests {
    use crate::{
        any_message::AnyMessageBox,
        skill::CharSkillProgressKind,
        skill_impl_utils::{SkillEffectUnit, SkillEffectUnits, UpdateUnits},
    };

    fn assert_eq_f32(a: f32, b: f32) {
        let diff = (a - b).abs();
        assert!(diff < 0.00001, "expect equal: a={} b={}", a, b);
    }

    #[test]
    fn test_tick() {
        let units_vec = vec![
            SkillEffectUnit::new(10, CharSkillProgressKind::Chanting, |_, _, _| {}).unwrap(),
            SkillEffectUnit::new(10, CharSkillProgressKind::Chanting, |_, _, _| {}).unwrap(),
            SkillEffectUnit::new(10, CharSkillProgressKind::Acting, |_, _, _| {}).unwrap(),
            SkillEffectUnit::new(10, CharSkillProgressKind::Acting, |_, _, _| {}).unwrap(),
        ];

        let mut units = SkillEffectUnits::new(units_vec).unwrap();
        units.start().unwrap();

        let (effect_fns, any_msg, ended) = units.tick_inner(9.0).unwrap();
        let msg = any_msg.downcast_ref::<UpdateUnits>().unwrap();
        assert_eq_f32(msg.add_progress, 9.0);
        assert_eq!(msg.steps, 0);
        assert_eq!(effect_fns.len(), 0);
        assert!(!ended);

        let (effect_fns, any_msg, ended) = units.tick_inner(10.1).unwrap();
        let msg = any_msg.downcast_ref::<UpdateUnits>().unwrap();
        assert_eq_f32(msg.add_progress, 10.1);
        assert_eq!(msg.steps, 1);
        assert_eq!(effect_fns.len(), 1);
        assert!(!ended);

        let (effect_fns, any_msg, ended) = units.tick_inner(20.1).unwrap();
        let msg = any_msg.downcast_ref::<UpdateUnits>().unwrap();
        assert_eq_f32(msg.add_progress, 20.1);
        assert_eq!(msg.steps, 2);
        assert_eq!(effect_fns.len(), 2);
        assert!(!ended);
    }

    #[test]
    fn test_tick2() {
        let units_vec = vec![
            SkillEffectUnit::new(10, CharSkillProgressKind::Chanting, |_, _, _| {}).unwrap(),
            SkillEffectUnit::new(10, CharSkillProgressKind::Chanting, |_, _, _| {}).unwrap(),
            SkillEffectUnit::new(10, CharSkillProgressKind::Acting, |_, _, _| {}).unwrap(),
            SkillEffectUnit::new(10, CharSkillProgressKind::Acting, |_, _, _| {}).unwrap(),
        ];

        let mut units = SkillEffectUnits::new(units_vec).unwrap();
        units.start().unwrap();

        let mut effect_fns_count = 0;
        let tick_args = [5.0, 5.0, 10.0, 8.0, 10.0, 10.0, 20.0];
        let mut last_msg: Option<AnyMessageBox> = None;
        for tick_arg in tick_args {
            let (effect_fns, any_msg, ended) = units.tick_inner(tick_arg).unwrap();
            effect_fns_count += effect_fns.len();
            if effect_fns_count == 4 {
                assert!(ended);
            } else {
                assert!(!ended);
            }
            let res = units.update(&any_msg);
            assert!(res.is_none());
            last_msg = Some(any_msg);
        }

        assert!(last_msg.unwrap().downcast_mut::<UpdateUnits>().is_some());
        assert_eq!(effect_fns_count, 4);
    }
}
