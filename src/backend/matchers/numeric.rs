use crate::backend::Assertion;
use crate::backend::assertions::sentence::AssertionSentence;
use std::fmt::{Debug, Display};
use std::ops::Range;

/// Trait for numeric assertions.
///
/// Provides matchers for comparing, classifying, and range-checking numeric values.
/// Supported for all standard numeric types: `i8`..`i128`, `u8`..`u128`, `isize`, `usize`, `f32`, `f64`.
pub trait NumericMatchers<T> {
    fn to_be_positive(self) -> Self;
    fn to_be_negative(self) -> Self;
    fn to_be_zero(self) -> Self;
    fn to_be_greater_than(self, expected: T) -> Self;
    fn to_be_greater_than_or_equal(self, expected: T) -> Self;
    fn to_be_less_than(self, expected: T) -> Self;
    fn to_be_less_than_or_equal(self, expected: T) -> Self;
    fn to_be_in_range(self, range: Range<T>) -> Self;
    fn to_be_even(self) -> Self;
    fn to_be_odd(self) -> Self;
}

/// Internal helper trait implemented by all supported numeric types.
trait Numeric: PartialOrd + PartialEq + Display + Clone + Copy {
    fn zero() -> Self;
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
    fn is_negative(&self) -> bool;
}

macro_rules! impl_numeric_signed {
    ($($t:ty),*) => {
        $(
            impl Numeric for $t {
                fn zero() -> Self { 0 }
                fn is_even(&self) -> bool { *self % 2 == 0 }
                fn is_odd(&self) -> bool { *self % 2 != 0 }
                fn is_negative(&self) -> bool { *self < 0 }
            }
        )*
    };
}

macro_rules! impl_numeric_unsigned {
    ($($t:ty),*) => {
        $(
            impl Numeric for $t {
                fn zero() -> Self { 0 }
                fn is_even(&self) -> bool { *self % 2 == 0 }
                fn is_odd(&self) -> bool { *self % 2 != 0 }
                fn is_negative(&self) -> bool { false }
            }
        )*
    };
}

macro_rules! impl_numeric_float {
    ($($t:ty),*) => {
        $(
            impl Numeric for $t {
                fn zero() -> Self { 0.0 }

                fn is_even(&self) -> bool {
                    return *self == self.trunc() && (*self as i64) % 2 == 0;
                }

                fn is_odd(&self) -> bool {
                    return *self == self.trunc() && (*self as i64) % 2 != 0;
                }

                fn is_negative(&self) -> bool { *self < 0.0 }
            }
        )*
    };
}

impl_numeric_signed!(i8, i16, i32, i64, i128, isize);
impl_numeric_unsigned!(u8, u16, u32, u64, u128, usize);
impl_numeric_float!(f32, f64);

/// Implementation for owned numeric values
impl<V> NumericMatchers<V> for Assertion<V>
where
    V: Numeric + Debug + Clone,
{
    fn to_be_positive(self) -> Self {
        let result = self.value > V::zero();
        let sentence = AssertionSentence::new("be", "positive");

        return self.add_step(sentence, result);
    }

    fn to_be_negative(self) -> Self {
        let result = self.value.is_negative();
        let sentence = AssertionSentence::new("be", "negative");

        return self.add_step(sentence, result);
    }

    fn to_be_zero(self) -> Self {
        let result = self.value == V::zero();
        let sentence = AssertionSentence::new("be", "zero");

        return self.add_step(sentence, result);
    }

    fn to_be_greater_than(self, expected: V) -> Self {
        let result = self.value > expected;
        let sentence = AssertionSentence::new("be", format!("greater than {}", expected));

        return self.add_step(sentence, result);
    }

    fn to_be_greater_than_or_equal(self, expected: V) -> Self {
        let result = self.value >= expected;
        let sentence = AssertionSentence::new("be", format!("greater than or equal to {}", expected));

        return self.add_step(sentence, result);
    }

    fn to_be_less_than(self, expected: V) -> Self {
        let result = self.value < expected;
        let sentence = AssertionSentence::new("be", format!("less than {}", expected));

        return self.add_step(sentence, result);
    }

    fn to_be_less_than_or_equal(self, expected: V) -> Self {
        let result = self.value <= expected;
        let sentence = AssertionSentence::new("be", format!("less than or equal to {}", expected));

        return self.add_step(sentence, result);
    }

    fn to_be_in_range(self, range: Range<V>) -> Self {
        let result = range.contains(&self.value);
        let sentence = AssertionSentence::new("be", format!("in range {}..{}", range.start, range.end));

        return self.add_step(sentence, result);
    }

    fn to_be_even(self) -> Self {
        let result = self.value.is_even();
        let sentence = AssertionSentence::new("be", "even");

        return self.add_step(sentence, result);
    }

    fn to_be_odd(self) -> Self {
        let result = self.value.is_odd();
        let sentence = AssertionSentence::new("be", "odd");

        return self.add_step(sentence, result);
    }
}

/// Implementation for referenced numeric values
impl<V> NumericMatchers<V> for Assertion<&V>
where
    V: Numeric + Debug + Clone,
{
    fn to_be_positive(self) -> Self {
        let result = *self.value > V::zero();
        let sentence = AssertionSentence::new("be", "positive");

        return self.add_step(sentence, result);
    }

    fn to_be_negative(self) -> Self {
        let result = self.value.is_negative();
        let sentence = AssertionSentence::new("be", "negative");

        return self.add_step(sentence, result);
    }

    fn to_be_zero(self) -> Self {
        let result = *self.value == V::zero();
        let sentence = AssertionSentence::new("be", "zero");

        return self.add_step(sentence, result);
    }

    fn to_be_greater_than(self, expected: V) -> Self {
        let result = *self.value > expected;
        let sentence = AssertionSentence::new("be", format!("greater than {}", expected));

        return self.add_step(sentence, result);
    }

    fn to_be_greater_than_or_equal(self, expected: V) -> Self {
        let result = *self.value >= expected;
        let sentence = AssertionSentence::new("be", format!("greater than or equal to {}", expected));

        return self.add_step(sentence, result);
    }

    fn to_be_less_than(self, expected: V) -> Self {
        let result = *self.value < expected;
        let sentence = AssertionSentence::new("be", format!("less than {}", expected));

        return self.add_step(sentence, result);
    }

    fn to_be_less_than_or_equal(self, expected: V) -> Self {
        let result = *self.value <= expected;
        let sentence = AssertionSentence::new("be", format!("less than or equal to {}", expected));

        return self.add_step(sentence, result);
    }

    fn to_be_in_range(self, range: Range<V>) -> Self {
        let result = range.contains(self.value);
        let sentence = AssertionSentence::new("be", format!("in range {}..{}", range.start, range.end));

        return self.add_step(sentence, result);
    }

    fn to_be_even(self) -> Self {
        let result = self.value.is_even();
        let sentence = AssertionSentence::new("be", "even");

        return self.add_step(sentence, result);
    }

    fn to_be_odd(self) -> Self {
        let result = self.value.is_odd();
        let sentence = AssertionSentence::new("be", "odd");

        return self.add_step(sentence, result);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_i32_matchers() {
        crate::Reporter::disable_deduplication();

        expect!(5_i32).to_be_positive();
        expect!(-5_i32).to_be_negative();
        expect!(0_i32).to_be_zero();
        expect!(5_i32).to_be_greater_than(2);
        expect!(5_i32).to_be_greater_than_or_equal(5);
        expect!(5_i32).to_be_less_than(10);
        expect!(5_i32).to_be_less_than_or_equal(5);
        expect!(5_i32).to_be_in_range(0..10);
        expect!(6_i32).to_be_even();
        expect!(7_i32).to_be_odd();

        // Negation
        expect!(5_i32).not().to_be_negative();
        expect!(-5_i32).not().to_be_positive();
        expect!(5_i32).not().to_be_zero();
        expect!(2_i32).not().to_be_greater_than(5);
        expect!(4_i32).not().to_be_greater_than_or_equal(5);
        expect!(10_i32).not().to_be_less_than(5);
        expect!(6_i32).not().to_be_less_than_or_equal(5);
        expect!(15_i32).not().to_be_in_range(0..10);
        expect!(7_i32).not().to_be_even();
        expect!(6_i32).not().to_be_odd();
    }

    #[test]
    fn test_i8_matchers() {
        crate::Reporter::disable_deduplication();

        expect!(5_i8).to_be_positive();
        expect!(-5_i8).to_be_negative();
        expect!(0_i8).to_be_zero();
        expect!(5_i8).to_be_greater_than(2_i8);
        expect!(5_i8).to_be_in_range(0_i8..10_i8);
        expect!(6_i8).to_be_even();
    }

    #[test]
    fn test_i64_matchers() {
        crate::Reporter::disable_deduplication();

        expect!(500_000_i64).to_be_positive();
        expect!(-500_000_i64).to_be_negative();
        expect!(0_i64).to_be_zero();
        expect!(500_000_i64).to_be_greater_than(200_000_i64);
        expect!(500_000_i64).to_be_in_range(0_i64..1_000_000_i64);
    }

    #[test]
    fn test_u32_matchers() {
        crate::Reporter::disable_deduplication();

        expect!(5_u32).to_be_positive();
        expect!(0_u32).to_be_zero();
        expect!(5_u32).to_be_greater_than(2_u32);
        expect!(5_u32).to_be_less_than(10_u32);
        expect!(5_u32).to_be_in_range(0_u32..10_u32);
        expect!(6_u32).to_be_even();
        expect!(7_u32).to_be_odd();
    }

    #[test]
    fn test_u64_matchers() {
        crate::Reporter::disable_deduplication();

        expect!(5_u64).to_be_positive();
        expect!(0_u64).to_be_zero();
        expect!(5_u64).to_be_greater_than(2_u64);
        expect!(100_u64).to_be_in_range(0_u64..200_u64);
        expect!(6_u64).to_be_even();
        expect!(7_u64).to_be_odd();
    }

    #[test]
    fn test_usize_matchers() {
        crate::Reporter::disable_deduplication();

        expect!(5_usize).to_be_positive();
        expect!(0_usize).to_be_zero();
        expect!(5_usize).to_be_greater_than(2_usize);
        expect!(5_usize).to_be_in_range(0_usize..10_usize);
        expect!(6_usize).to_be_even();
        expect!(7_usize).to_be_odd();
    }

    #[test]
    fn test_unsigned_not_negative() {
        crate::Reporter::disable_deduplication();

        // Unsigned types can never be negative
        expect!(5_u32).not().to_be_negative();
        expect!(0_u32).not().to_be_negative();
        expect!(5_u64).not().to_be_negative();
        expect!(0_usize).not().to_be_negative();
    }

    #[test]
    fn test_f64_matchers() {
        crate::Reporter::disable_deduplication();

        expect!(3.14_f64).to_be_positive();
        expect!(-2.5_f64).to_be_negative();
        expect!(0.0_f64).to_be_zero();
        expect!(5.5_f64).to_be_greater_than(2.0_f64);
        expect!(5.5_f64).to_be_greater_than_or_equal(5.5_f64);
        expect!(5.5_f64).to_be_less_than(10.0_f64);
        expect!(5.5_f64).to_be_less_than_or_equal(5.5_f64);
        expect!(3.14_f64).to_be_in_range(0.0_f64..10.0_f64);
    }

    #[test]
    fn test_f32_matchers() {
        crate::Reporter::disable_deduplication();

        expect!(3.14_f32).to_be_positive();
        expect!(-2.5_f32).to_be_negative();
        expect!(0.0_f32).to_be_zero();
        expect!(5.5_f32).to_be_greater_than(2.0_f32);
        expect!(3.14_f32).to_be_in_range(0.0_f32..10.0_f32);
    }

    #[test]
    fn test_float_even_odd() {
        crate::Reporter::disable_deduplication();

        // Whole-number floats support even/odd checks
        expect!(4.0_f64).to_be_even();
        expect!(3.0_f64).to_be_odd();
        expect!(4.0_f64).not().to_be_odd();
        expect!(3.0_f64).not().to_be_even();

        // Non-whole floats are neither even nor odd
        expect!(3.14_f64).not().to_be_even();
        expect!(3.14_f64).not().to_be_odd();
    }

    #[test]
    fn test_reference_matchers() {
        crate::Reporter::disable_deduplication();

        let val: i32 = 42;
        expect!(&val).to_be_positive();
        expect!(&val).to_be_greater_than(10);
        expect!(&val).to_be_in_range(0..100);
        expect!(&val).to_be_even();

        let fval: f64 = 3.14;
        expect!(&fval).to_be_positive();
        expect!(&fval).to_be_greater_than(1.0);
    }

    #[test]
    fn test_default_integer_literals() {
        crate::Reporter::disable_deduplication();

        // Bare integer literals default to i32
        expect!(5).to_be_positive();
        expect!(-5).to_be_negative();
        expect!(0).to_be_zero();
        expect!(5).to_be_greater_than(2);
        expect!(5).to_be_in_range(0..10);
        expect!(6).to_be_even();
        expect!(7).to_be_odd();
    }

    #[test]
    #[should_panic(expected = "be positive")]
    fn test_not_positive_fails() {
        expect!(-5).to_be_positive();
    }

    #[test]
    #[should_panic(expected = "be negative")]
    fn test_not_negative_fails() {
        expect!(5).to_be_negative();
    }

    #[test]
    #[should_panic(expected = "be zero")]
    fn test_not_zero_fails() {
        expect!(5).to_be_zero();
    }

    #[test]
    #[should_panic(expected = "be greater than")]
    fn test_not_greater_fails() {
        expect!(2).to_be_greater_than(5);
    }

    #[test]
    #[should_panic(expected = "be greater than or equal to")]
    fn test_not_greater_equal_fails() {
        expect!(4).to_be_greater_than_or_equal(5);
    }

    #[test]
    #[should_panic(expected = "be less than")]
    fn test_not_less_fails() {
        expect!(10).to_be_less_than(5);
    }

    #[test]
    #[should_panic(expected = "be less than or equal to")]
    fn test_not_less_equal_fails() {
        expect!(6).to_be_less_than_or_equal(5);
    }

    #[test]
    #[should_panic(expected = "be in range")]
    fn test_not_in_range_fails() {
        expect!(15).to_be_in_range(0..10);
    }

    #[test]
    #[should_panic(expected = "be even")]
    fn test_not_even_fails() {
        expect!(7).to_be_even();
    }

    #[test]
    #[should_panic(expected = "be odd")]
    fn test_not_odd_fails() {
        expect!(6).to_be_odd();
    }

    #[test]
    #[should_panic(expected = "be negative")]
    fn test_unsigned_negative_fails() {
        expect!(5_u32).to_be_negative();
    }

    #[test]
    #[should_panic(expected = "be positive")]
    fn test_f64_not_positive_fails() {
        expect!(-3.14_f64).to_be_positive();
    }

    #[test]
    #[should_panic(expected = "be greater than")]
    fn test_f64_not_greater_fails() {
        expect!(2.0_f64).to_be_greater_than(5.0_f64);
    }
}
