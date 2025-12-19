#[derive(Debug, Clone, Copy)]
pub struct ProgressState {
    current: f32,
    max: f32,
}

impl ProgressState {
    pub fn new<T: Into<f32>>(current: T, max: T) -> Option<Self> {
        let current = current.into();
        let max = max.into();

        if current > max {
            return None;
        }

        if current < 0.0 || max <= 0.0 {
            return None;
        }

        if !current.is_finite() || !max.is_finite() {
            return None;
        }

        Some(Self { current, max })
    }

    /// Returns a value between 0.0 and 1.0 representing the progress.
    pub fn progress(&self) -> f32 {
        let p = self.current / self.max;

        assert!(p >= 0.0);
        assert!(p <= 1.0);

        p
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn current(&self) -> f32 {
        self.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid_values() {
        let state = ProgressState::new(3.0, 10.0).unwrap();
        assert_eq!(state.current(), 3.0);
        assert_eq!(state.max(), 10.0);
    }

    #[test]
    fn progress_zero() {
        let state = ProgressState::new(0.0, 10.0).unwrap();
        assert_eq!(state.progress(), 0.0);
    }

    #[test]
    fn progress_half() {
        let state = ProgressState::new(5.0, 10.0).unwrap();
        assert_eq!(state.progress(), 0.5);
    }

    #[test]
    fn progress_full() {
        let state = ProgressState::new(10.0, 10.0).unwrap();
        assert_eq!(state.progress(), 1.0);
    }

    #[test]
    fn new_current_greater_than_max_returns_none() {
        assert!(ProgressState::new(11.0, 10.0).is_none());
    }

    #[test]
    fn new_negative_current_returns_none() {
        assert!(ProgressState::new(-1.0, 10.0).is_none());
    }

    #[test]
    fn new_zero_max_returns_none() {
        assert!(ProgressState::new(0.0, 0.0).is_none());
    }

    #[test]
    fn new_negative_max_returns_none() {
        assert!(ProgressState::new(1.0, -10.0).is_none());
    }

    #[test]
    fn new_nan_current_returns_none() {
        assert!(ProgressState::new(f32::NAN, 10.0).is_none());
    }

    #[test]
    fn new_nan_max_returns_none() {
        assert!(ProgressState::new(1.0, f32::NAN).is_none());
    }

    #[test]
    fn new_infinite_current_returns_none() {
        assert!(ProgressState::new(f32::INFINITY, 10.0).is_none());
    }

    #[test]
    fn new_infinite_max_returns_none() {
        assert!(ProgressState::new(1.0, f32::INFINITY).is_none());
    }

    #[test]
    fn progress_never_panics_for_valid_state() {
        let values = [(0.0, 1.0), (0.5, 1.0), (1.0, 1.0), (2.5, 10.0)];

        for (current, max) in values {
            let state = ProgressState::new(current, max).unwrap();
            let p = state.progress();
            assert!((0.0..=1.0).contains(&p));
        }
    }
}
