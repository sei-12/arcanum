pub mod round_digits {

    /// `f32` を指定した桁数で四捨五入するためのトレイト。
    ///
    /// # Overview
    ///
    /// `round_digits()` を使うと、`f32` の値を小数点以下や 10 の位など、
    /// 任意の桁数で四捨五入できます。
    ///
    /// - `digits >= 0` : 小数点以下の桁に丸める  
    /// - `digits < 0`  : 整数部側（10 の位 / 100 の位など）に丸める
    ///
    /// # Panics
    ///
    /// `digits` が `-38..=38` の範囲外の場合、丸めに必要な倍率が
    /// `f32` の範囲を超えるためパニックします.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::RoundDigits;
    ///
    /// assert_eq!(3.14159_f32.round_digits(2), 3.14_f32); // 小数2桁
    /// assert_eq!(123.45_f32.round_digits(-1), 120.0_f32); // 10の位
    /// assert_eq!((-1.235_f32).round_digits(2), -1.24_f32); // 負数の丸め
    /// ```
    pub trait RoundDigits {
        /// 値を指定した桁数に四捨五入する。
        ///
        /// # Examples
        ///
        /// ```
        /// assert_eq!(1.23456_f32.round_digits(3), 1.235_f32);
        /// assert_eq!(156.0_f32.round_digits(-1), 160.0_f32);
        /// assert_eq!((-1.235_f32).round_digits(2), -1.24_f32);
        /// ```
        ///
        /// # Arguments
        /// - `digits`: 丸めたい桁数  
        ///   - `digits >= 0`: 小数点以下 `digits` 桁で丸める  
        ///   - `digits < 0`: 10 の位・100 の位などで丸める
        ///
        /// # Behavior
        /// - `NaN` と `±∞` は変更せずそのまま返します。
        ///
        /// # Panics
        /// `digits` が `-38..=38` の範囲外の場合はパニックします。
        ///
        fn round_digits(self, digits: i32) -> Self;
    }

    impl RoundDigits for f32 {
        fn round_digits(self, digits: i32) -> Self {
            assert!((-38..=38).contains(&digits));

            if !self.is_finite() {
                return self;
            }

            if digits >= 0 {
                let m = 10f32.powi(digits);
                (self * m).round() / m
            } else {
                let m = 10f32.powi(-digits);
                (self / m).round() * m
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::RoundDigits;

        #[test]
        fn test_round_positive_digits() {
            assert_eq!(1.23456_f32.round_digits(2), 1.23_f32);
            assert_eq!(1.23556_f32.round_digits(2), 1.24_f32);
            assert_eq!((-1.23556_f32).round_digits(2), -1.24_f32);
        }

        #[test]
        fn test_round_negative_digits() {
            assert_eq!(123.45_f32.round_digits(-1), 120.0_f32);
            assert_eq!(155.0_f32.round_digits(-2), 200.0_f32);
        }

        #[test]
        fn test_special_values() {
            assert!(f32::NAN.round_digits(2).is_nan());
            assert_eq!(f32::INFINITY.round_digits(3), f32::INFINITY);
            assert_eq!(f32::NEG_INFINITY.round_digits(3), f32::NEG_INFINITY);
        }

        #[test]
        #[should_panic]
        fn test_digits_too_large() {
            1.23_f32.round_digits(100);
        }

        #[test]
        #[should_panic]
        fn test_digits_too_small() {
            1.23_f32.round_digits(-100);
        }
    }
}
