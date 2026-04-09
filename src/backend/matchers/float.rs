use crate::backend::Assertion;
use crate::backend::assertions::sentence::AssertionSentence;
use std::fmt::Debug;

/// Trait for float-specific assertions.
///
/// Provides matchers for approximate equality and special float value classification.
/// Supported for `f32` and `f64`.
pub trait FloatMatchers<T> {
    fn to_be_close_to(self, expected: T, tolerance: T) -> Self;
    fn to_be_nan(self) -> Self;
    fn to_be_infinite(self) -> Self;
    fn to_be_finite(self) -> Self;
}

/// Internal helper trait implemented by float types.
trait AsFloat: PartialOrd + Copy {
    fn is_nan_value(&self) -> bool;
    fn is_infinite_value(&self) -> bool;
    fn is_finite_value(&self) -> bool;
    fn is_close_to(&self, expected: Self, tolerance: Self) -> bool;
    fn display_value(&self) -> String;
}

impl AsFloat for f32 {
    fn is_nan_value(&self) -> bool {
        self.is_nan()
    }

    fn is_infinite_value(&self) -> bool {
        self.is_infinite()
    }

    fn is_finite_value(&self) -> bool {
        self.is_finite()
    }

    fn is_close_to(&self, expected: f32, tolerance: f32) -> bool {
        (self - expected).abs() <= tolerance
    }

    fn display_value(&self) -> String {
        format!("{}", self)
    }
}

impl AsFloat for f64 {
    fn is_nan_value(&self) -> bool {
        self.is_nan()
    }

    fn is_infinite_value(&self) -> bool {
        self.is_infinite()
    }

    fn is_finite_value(&self) -> bool {
        self.is_finite()
    }

    fn is_close_to(&self, expected: f64, tolerance: f64) -> bool {
        (self - expected).abs() <= tolerance
    }

    fn display_value(&self) -> String {
        format!("{}", self)
    }
}

impl AsFloat for &f32 {
    fn is_nan_value(&self) -> bool {
        (**self).is_nan()
    }

    fn is_infinite_value(&self) -> bool {
        (**self).is_infinite()
    }

    fn is_finite_value(&self) -> bool {
        (**self).is_finite()
    }

    fn is_close_to(&self, expected: &f32, tolerance: &f32) -> bool {
        (**self - *expected).abs() <= *tolerance
    }

    fn display_value(&self) -> String {
        format!("{}", **self)
    }
}

impl AsFloat for &f64 {
    fn is_nan_value(&self) -> bool {
        (**self).is_nan()
    }

    fn is_infinite_value(&self) -> bool {
        (**self).is_infinite()
    }

    fn is_finite_value(&self) -> bool {
        (**self).is_finite()
    }

    fn is_close_to(&self, expected: &f64, tolerance: &f64) -> bool {
        (**self - *expected).abs() <= *tolerance
    }

    fn display_value(&self) -> String {
        format!("{}", **self)
    }
}

/// Implementation for owned float values
impl<V> FloatMatchers<V> for Assertion<V>
where
    V: AsFloat + Debug + Clone,
{
    fn to_be_close_to(self, expected: V, tolerance: V) -> Self {
        let result = self.value.is_close_to(expected, tolerance);
        let sentence = AssertionSentence::new("be", format!("close to {} ± {}", expected.display_value(), tolerance.display_value()))
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_be_nan(self) -> Self {
        let result = self.value.is_nan_value();
        let sentence = AssertionSentence::new("be", "NaN").with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_be_infinite(self) -> Self {
        let result = self.value.is_infinite_value();
        let sentence = AssertionSentence::new("be", "infinite").with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_be_finite(self) -> Self {
        let result = self.value.is_finite_value();
        let sentence = AssertionSentence::new("be", "finite").with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    // ── to_be_close_to ──────────────────────────────────────────────

    #[test]
    fn test_f64_close_to() {
        crate::Reporter::disable_deduplication();

        expect!(0.1 + 0.2).to_be_close_to(0.3, 0.001);
    }

    #[test]
    fn test_f32_close_to() {
        crate::Reporter::disable_deduplication();

        let result: f32 = 0.1_f32 + 0.2_f32;
        expect!(result).to_be_close_to(0.3_f32, 0.001_f32);
    }

    #[test]
    fn test_close_to_with_very_small_tolerance() {
        crate::Reporter::disable_deduplication();

        let pi: f64 = std::f64::consts::PI;
        expect!(pi).to_be_close_to(3.14159265358979, 0.00000000000001);
    }

    #[test]
    #[should_panic(expected = "be close to")]
    fn test_close_to_failure() {
        let _assertion = expect!(1.0_f64).to_be_close_to(2.0, 0.001);
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_close_to_exact_match() {
        crate::Reporter::disable_deduplication();

        expect!(1.0_f64).to_be_close_to(1.0, 0.0);
    }

    // ── to_be_nan ───────────────────────────────────────────────────

    #[test]
    fn test_nan_f64() {
        crate::Reporter::disable_deduplication();

        expect!(f64::NAN).to_be_nan();
    }

    #[test]
    fn test_nan_f32() {
        crate::Reporter::disable_deduplication();

        expect!(f32::NAN).to_be_nan();
    }

    #[test]
    #[should_panic(expected = "be NaN")]
    fn test_non_nan_fails() {
        let _assertion = expect!(1.0_f64).to_be_nan();
        std::hint::black_box(_assertion);
    }

    // ── to_be_infinite ──────────────────────────────────────────────

    #[test]
    fn test_positive_infinity() {
        crate::Reporter::disable_deduplication();

        expect!(f64::INFINITY).to_be_infinite();
    }

    #[test]
    fn test_negative_infinity() {
        crate::Reporter::disable_deduplication();

        expect!(f64::NEG_INFINITY).to_be_infinite();
    }

    #[test]
    fn test_f32_infinity() {
        crate::Reporter::disable_deduplication();

        expect!(f32::INFINITY).to_be_infinite();
    }

    #[test]
    #[should_panic(expected = "be infinite")]
    fn test_finite_is_not_infinite() {
        let _assertion = expect!(1.0_f64).to_be_infinite();
        std::hint::black_box(_assertion);
    }

    // ── to_be_finite ────────────────────────────────────────────────

    #[test]
    fn test_finite_value() {
        crate::Reporter::disable_deduplication();

        expect!(1.0_f64).to_be_finite();
        expect!(0.0_f32).to_be_finite();
        expect!(-42.5_f64).to_be_finite();
    }

    #[test]
    #[should_panic(expected = "be finite")]
    fn test_infinity_is_not_finite() {
        let _assertion = expect!(f64::INFINITY).to_be_finite();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "be finite")]
    fn test_nan_is_not_finite() {
        let _assertion = expect!(f64::NAN).to_be_finite();
        std::hint::black_box(_assertion);
    }

    // ── negation (.not()) ───────────────────────────────────────────

    #[test]
    fn test_not_nan() {
        crate::Reporter::disable_deduplication();

        expect!(1.0_f64).not().to_be_nan();
    }

    #[test]
    fn test_not_close_to() {
        crate::Reporter::disable_deduplication();

        expect!(1.0_f64).not().to_be_close_to(2.0, 0.001);
    }

    #[test]
    fn test_not_infinite() {
        crate::Reporter::disable_deduplication();

        expect!(1.0_f64).not().to_be_infinite();
    }

    #[test]
    fn test_not_finite() {
        crate::Reporter::disable_deduplication();

        expect!(f64::NAN).not().to_be_finite();
        expect!(f64::INFINITY).not().to_be_finite();
    }

    #[test]
    #[should_panic(expected = "not be NaN")]
    fn test_not_nan_fails_on_nan() {
        let _assertion = expect!(f64::NAN).not().to_be_nan();
        std::hint::black_box(_assertion);
    }

    // ── chaining (.and()) ───────────────────────────────────────────

    #[test]
    fn test_close_to_and_positive() {
        crate::Reporter::disable_deduplication();

        expect!(3.14_f64).to_be_close_to(std::f64::consts::PI, 0.01).and().to_be_positive();
    }

    #[test]
    fn test_close_to_and_finite() {
        crate::Reporter::disable_deduplication();

        expect!(0.1_f64 + 0.2_f64).to_be_close_to(0.3, 0.001).and().to_be_finite();
    }

    // ── edge cases ──────────────────────────────────────────────────

    #[test]
    fn test_negative_zero() {
        crate::Reporter::disable_deduplication();

        expect!(-0.0_f64).to_be_close_to(0.0, 0.0);
        expect!(-0.0_f64).to_be_finite();
        expect!(-0.0_f64).not().to_be_nan();
    }

    #[test]
    fn test_nan_close_to_nan() {
        crate::Reporter::disable_deduplication();

        // NaN is never close to anything, even itself
        expect!(f64::NAN).not().to_be_close_to(f64::NAN, 1.0);
    }

    #[test]
    fn test_infinity_not_close_to_finite() {
        crate::Reporter::disable_deduplication();

        expect!(f64::INFINITY).not().to_be_close_to(1.0, 1000.0);
    }
}
