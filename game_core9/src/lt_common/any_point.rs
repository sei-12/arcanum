#[derive(Debug, Clone, Copy)]
enum Num {
    /// 0.0 ~ 1.0
    Plus(f32),
    /// 実数値
    Minus(f32),
}

#[derive(Debug, Clone, Copy)]
pub struct AnyPointPercent {
    inner_val: Num,
}

impl AnyPointPercent {
    pub fn new_max() -> Self {
        Self {
            inner_val: Num::Plus(1.0),
        }
    }

    pub fn get(&self, max_point: f32) -> f32 {
        match self.inner_val {
            Num::Plus(v) => v * max_point,
            Num::Minus(v) => v,
        }
    }

    pub fn add(&mut self, max_point: f32, n: f32) {
        let tmp = self.get(max_point) + n;

        if tmp < 0.0 {
            self.inner_val = Num::Minus(tmp)
        } else {
            let per = tmp / max_point;
            if per > 1.0 {
                self.inner_val = Num::Plus(1.0)
            } else {
                self.inner_val = Num::Plus(per)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX: f32 = 100.0;
    const EPS: f32 = 1e-6;

    fn assert_f32_eq(a: f32, b: f32) {
        assert!((a - b).abs() < EPS, "assertion failed: {} != {}", a, b);
    }

    #[test]
    fn new_max_is_full_point() {
        let p = AnyPointPercent::new_max();
        assert_f32_eq(p.get(MAX), MAX);
    }

    #[test]
    fn add_zero_does_not_change_value() {
        let mut p = AnyPointPercent::new_max();
        p.add(MAX, 0.0);

        assert_f32_eq(p.get(MAX), MAX);
    }

    #[test]
    fn exact_zero_boundary() {
        let mut p = AnyPointPercent::new_max();

        // 100 -> 0
        p.add(MAX, -100.0);
        assert_f32_eq(p.get(MAX), 0.0);
    }

    #[test]
    fn exact_one_boundary() {
        let mut p = AnyPointPercent::new_max();

        // 一度下げてから元に戻す
        p.add(MAX, -30.0); // 70
        p.add(MAX, 30.0); // 100

        assert_f32_eq(p.get(MAX), MAX);
    }

    #[test]
    fn clamp_above_max() {
        let mut p = AnyPointPercent::new_max();

        // 明らかに超過
        p.add(MAX, 500.0);
        assert_f32_eq(p.get(MAX), MAX);
    }

    #[test]
    fn minus_state_keeps_raw_value() {
        let mut p = AnyPointPercent::new_max();

        p.add(MAX, -130.0); // -30
        assert_f32_eq(p.get(MAX), -30.0);

        // max_point を変えても影響を受けない
        assert_f32_eq(p.get(1000.0), -30.0);
    }

    #[test]
    fn minus_to_plus_transition() {
        let mut p = AnyPointPercent::new_max();

        p.add(MAX, -150.0); // -50
        p.add(MAX, 70.0); // 20

        assert_f32_eq(p.get(MAX), 20.0);
    }

    #[test]
    fn repeated_small_additions() {
        let mut p = AnyPointPercent::new_max();

        for _ in 0..10 {
            p.add(MAX, -5.0);
        }

        assert_f32_eq(p.get(MAX), 50.0);
    }

    #[test]
    fn repeated_crossing_zero() {
        let mut p = AnyPointPercent::new_max();

        p.add(MAX, -120.0); // -20
        p.add(MAX, 10.0); // -10
        p.add(MAX, 10.0); // 0
        p.add(MAX, 10.0); // 10

        assert_f32_eq(p.get(MAX), 10.0);
    }

    #[test]
    fn max_point_scaling_works() {
        let mut p = AnyPointPercent::new_max();

        p.add(200.0, -50.0); // 150 / 200 = 0.75

        assert_f32_eq(p.get(200.0), 150.0);
        assert_f32_eq(p.get(100.0), 75.0);
    }

    #[test]
    fn max_point_change_preserves_ratio() {
        let mut p = AnyPointPercent::new_max();

        // max=100 の世界で 80 にする → 0.8
        p.add(100.0, -20.0);
        assert_f32_eq(p.get(100.0), 80.0);

        // max を変更しても比率は維持される
        assert_f32_eq(p.get(200.0), 160.0);
        assert_f32_eq(p.get(50.0), 40.0);
    }

    #[test]
    fn add_with_different_max_point_updates_ratio_correctly() {
        let mut p = AnyPointPercent::new_max();

        // max=100 で 50 にする（0.5）
        p.add(100.0, -50.0);
        assert_f32_eq(p.get(100.0), 50.0);

        // +0.25
        p.add(200.0, 50.0);

        assert_f32_eq(p.get(200.0), 150.0);
        assert_f32_eq(p.get(100.0), 75.0);
    }

    #[test]
    fn minus_value_is_independent_of_max_point() {
        let mut p = AnyPointPercent::new_max();

        // Minus 状態を作る
        p.add(100.0, -130.0); // -30
        assert_f32_eq(p.get(100.0), -30.0);

        // max_point を変更しても値は変わらない
        assert_f32_eq(p.get(10.0), -30.0);
        assert_f32_eq(p.get(1_000.0), -30.0);
    }
}

