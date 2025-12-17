#[derive(Debug, Clone, Copy)]
pub struct ProgressState {
    current: f32,
    max: f32,
}

impl ProgressState {
    pub fn new<T: Into<f32>>(max: T, current: T) -> Option<Self> {
        let current = current.into();
        let max = max.into();

        if current > max {
            return None;
        }
        if current < 0.0 || max < 0.0 {
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
