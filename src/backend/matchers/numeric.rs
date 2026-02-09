use crate::backend::Assertion;
use crate::backend::assertions::sentence::AssertionSentence;
use std::fmt::Debug;
use std::ops::Range;

/// Trait for numeric assertions
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

/// Helper trait for numeric-like types
trait AsNumeric {
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;
    fn is_zero(&self) -> bool;
    fn is_greater_than(&self, expected: i32) -> bool;
    fn is_greater_than_or_equal(&self, expected: i32) -> bool;
    fn is_less_than(&self, expected: i32) -> bool;
    fn is_less_than_or_equal(&self, expected: i32) -> bool;
    fn is_in_range(&self, range: Range<i32>) -> bool;
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
}

// Implementation for i32
impl AsNumeric for i32 {
    fn is_positive(&self) -> bool {
        *self > 0
    }

    fn is_negative(&self) -> bool {
        *self < 0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_greater_than(&self, expected: i32) -> bool {
        *self > expected
    }

    fn is_greater_than_or_equal(&self, expected: i32) -> bool {
        *self >= expected
    }

    fn is_less_than(&self, expected: i32) -> bool {
        *self < expected
    }

    fn is_less_than_or_equal(&self, expected: i32) -> bool {
        *self <= expected
    }

    fn is_in_range(&self, range: Range<i32>) -> bool {
        range.contains(self)
    }

    fn is_even(&self) -> bool {
        *self % 2 == 0
    }

    fn is_odd(&self) -> bool {
        *self % 2 != 0
    }
}

// Implementation for &i32
impl AsNumeric for &i32 {
    fn is_positive(&self) -> bool {
        **self > 0
    }

    fn is_negative(&self) -> bool {
        **self < 0
    }

    fn is_zero(&self) -> bool {
        **self == 0
    }

    fn is_greater_than(&self, expected: i32) -> bool {
        **self > expected
    }

    fn is_greater_than_or_equal(&self, expected: i32) -> bool {
        **self >= expected
    }

    fn is_less_than(&self, expected: i32) -> bool {
        **self < expected
    }

    fn is_less_than_or_equal(&self, expected: i32) -> bool {
        **self <= expected
    }

    fn is_in_range(&self, range: Range<i32>) -> bool {
        range.contains(*self)
    }

    fn is_even(&self) -> bool {
        **self % 2 == 0
    }

    fn is_odd(&self) -> bool {
        **self % 2 != 0
    }
}

// Implementation for usize
impl AsNumeric for usize {
    fn is_positive(&self) -> bool {
        *self > 0
    }

    fn is_negative(&self) -> bool {
        false // usize cannot be negative
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_greater_than(&self, expected: i32) -> bool {
        *self > expected as usize
    }

    fn is_greater_than_or_equal(&self, expected: i32) -> bool {
        *self >= expected as usize
    }

    fn is_less_than(&self, expected: i32) -> bool {
        *self < expected as usize
    }

    fn is_less_than_or_equal(&self, expected: i32) -> bool {
        *self <= expected as usize
    }

    fn is_in_range(&self, range: Range<i32>) -> bool {
        range.contains(&(*self as i32))
    }

    fn is_even(&self) -> bool {
        (*self).is_multiple_of(2)
    }

    fn is_odd(&self) -> bool {
        !(*self).is_multiple_of(2)
    }
}

// Implementation for &usize
impl AsNumeric for &usize {
    fn is_positive(&self) -> bool {
        **self > 0
    }

    fn is_negative(&self) -> bool {
        false // usize cannot be negative
    }

    fn is_zero(&self) -> bool {
        **self == 0
    }

    fn is_greater_than(&self, expected: i32) -> bool {
        **self > expected as usize
    }

    fn is_greater_than_or_equal(&self, expected: i32) -> bool {
        **self >= expected as usize
    }

    fn is_less_than(&self, expected: i32) -> bool {
        **self < expected as usize
    }

    fn is_less_than_or_equal(&self, expected: i32) -> bool {
        **self <= expected as usize
    }

    fn is_in_range(&self, range: Range<i32>) -> bool {
        range.contains(&(**self as i32))
    }

    fn is_even(&self) -> bool {
        (**self).is_multiple_of(2)
    }

    fn is_odd(&self) -> bool {
        !(**self).is_multiple_of(2)
    }
}

// Single implementation for any type that implements AsNumeric
impl<V> NumericMatchers<i32> for Assertion<V>
where
    V: AsNumeric + Debug + Clone,
{
    fn to_be_positive(self) -> Self {
        let result = self.value.is_positive();
        let sentence = AssertionSentence::new("be", "positive");

        return self.add_step(sentence, result);
    }

    fn to_be_negative(self) -> Self {
        let result = self.value.is_negative();
        let sentence = AssertionSentence::new("be", "negative");

        return self.add_step(sentence, result);
    }

    fn to_be_zero(self) -> Self {
        let result = self.value.is_zero();
        let sentence = AssertionSentence::new("be", "zero");

        return self.add_step(sentence, result);
    }

    fn to_be_greater_than(self, expected: i32) -> Self {
        let result = self.value.is_greater_than(expected);
        let sentence = AssertionSentence::new("be", format!("greater than {}", expected));

        return self.add_step(sentence, result);
    }

    fn to_be_greater_than_or_equal(self, expected: i32) -> Self {
        let result = self.value.is_greater_than_or_equal(expected);
        let sentence = AssertionSentence::new("be", format!("greater than or equal to {}", expected));

        return self.add_step(sentence, result);
    }

    fn to_be_less_than(self, expected: i32) -> Self {
        let result = self.value.is_less_than(expected);
        let sentence = AssertionSentence::new("be", format!("less than {}", expected));

        return self.add_step(sentence, result);
    }

    fn to_be_less_than_or_equal(self, expected: i32) -> Self {
        let result = self.value.is_less_than_or_equal(expected);
        let sentence = AssertionSentence::new("be", format!("less than or equal to {}", expected));

        return self.add_step(sentence, result);
    }

    fn to_be_in_range(self, range: Range<i32>) -> Self {
        let result = self.value.is_in_range(range.clone());
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
    fn test_numeric_matchers() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass
        expect!(5).to_be_positive();
        expect!(-5).to_be_negative();
        expect!(0).to_be_zero();
        expect!(5).to_be_greater_than(2);
        expect!(5).to_be_greater_than_or_equal(5);
        expect!(5).to_be_less_than(10);
        expect!(5).to_be_less_than_or_equal(5);
        expect!(5).to_be_in_range(0..10);
        expect!(6).to_be_even();
        expect!(7).to_be_odd();

        // These with negation should also pass
        expect!(5).not().to_be_negative();
        expect!(-5).not().to_be_positive();
        expect!(5).not().to_be_zero();
        expect!(2).not().to_be_greater_than(5);
        expect!(4).not().to_be_greater_than_or_equal(5);
        expect!(10).not().to_be_less_than(5);
        expect!(6).not().to_be_less_than_or_equal(5);
        expect!(15).not().to_be_in_range(0..10);
        expect!(7).not().to_be_even();
        expect!(6).not().to_be_odd();
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
}
