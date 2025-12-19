use crate::{
    progress_state::ProgressState, skill::CharSkillProgressKind, skill_impl_utils::SkillEffectUnit,
};

#[derive(Debug, Clone)]
pub struct SkillChunks {
    chunks: Vec<(CharSkillProgressKind, f32)>,
}

impl SkillChunks {
    pub fn new(units: &[SkillEffectUnit]) -> Self {
        assert!(!units.is_empty());

        let mut chunks = Vec::new();

        for i in units {
            if let Some((last_kind, last_times)) = chunks.last_mut()
                && *last_kind == i.progress_kind
            {
                *last_times += i.time_ms as f32;
            } else {
                chunks.push((i.progress_kind, i.time_ms as f32));
            }
        }

        assert!(!chunks.is_empty());
        debug_assert!({ chunks.iter().all(|(_, t)| t.is_finite() && *t > 0.0) });

        Self { chunks }
    }

    /// 超重要: もし進捗が100%の場合、最後の要素のkindを返す
    ///
    /// 進捗が100%の場合はこの関数は呼ばれることはないはずだが
    /// f32の誤差でそうとも限らないと思うのでこのような仕様にする
    pub fn current_chunk_progress(
        &self,
        current_progress: f32,
    ) -> (CharSkillProgressKind, ProgressState) {
        assert!(current_progress.is_finite());
        assert!(current_progress >= 0.0);

        let mut tmp = current_progress;

        for (kind, time) in &self.chunks {
            if tmp <= *time {
                return (*kind, ProgressState::new(tmp, *time).unwrap());
            }
            tmp -= time;
        }

        let last_item = self.chunks.last().unwrap();
        (
            last_item.0,
            ProgressState::new(last_item.1, last_item.1).unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        skill::CharSkillProgressKind,
        skill_impl_utils::{SkillEffectUnit, skill_chunks::SkillChunks},
    };

    fn assert_eq_f32(a: f32, b: f32) {
        let diff = (a - b).abs();
        assert!(diff < 0.00001, "expect equal: a={} b={}", a, b);
    }

    fn make_chunks() -> SkillChunks {
        let units = [
            SkillEffectUnit::new(10, CharSkillProgressKind::Chanting, |_, _, _| {}).unwrap(),
            SkillEffectUnit::new(10, CharSkillProgressKind::Chanting, |_, _, _| {}).unwrap(),
            SkillEffectUnit::new(10, CharSkillProgressKind::Acting, |_, _, _| {}).unwrap(),
            SkillEffectUnit::new(10, CharSkillProgressKind::Acting, |_, _, _| {}).unwrap(),
        ];
        SkillChunks::new(&units)
    }

    #[test]
    fn chunk_generation_merges_same_kind() {
        let chunks = make_chunks();
        assert_eq!(chunks.chunks.len(), 2);
    }

    #[test]
    fn progress_within_first_chunk() {
        let chunks = make_chunks();

        let (kind, progress) = chunks.current_chunk_progress(10.0);
        assert_eq!(kind, CharSkillProgressKind::Chanting);
        assert_eq_f32(progress.progress(), 0.5);

        let (kind, progress) = chunks.current_chunk_progress(19.9);
        assert_eq!(kind, CharSkillProgressKind::Chanting);
        assert_eq_f32(progress.progress(), 19.9 / 20.0);
    }

    #[test]
    fn progress_across_chunk_boundary() {
        let chunks = make_chunks();

        let (kind, progress) = chunks.current_chunk_progress(20.1);
        assert_eq!(kind, CharSkillProgressKind::Acting);
        assert_eq_f32(progress.progress(), 0.1 / 20.0);
    }

    #[test]
    fn progress_over_100_percent_returns_last_chunk() {
        let chunks = make_chunks();

        let (kind, progress) = chunks.current_chunk_progress(100.0);
        assert_eq!(kind, CharSkillProgressKind::Acting);
        assert_eq_f32(progress.progress(), 1.0);
    }
}
