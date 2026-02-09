use crate::backend::Assertion;
use crate::backend::assertions::sentence::AssertionSentence;
use std::fmt::Debug;

pub trait EqualityMatchers<T> {
    /// Check if the value is equal to the expected value
    fn to_equal(self, expected: T) -> Self;

    /// Type-specific version of to_equal to avoid trait conflicts
    fn to_equal_value(self, expected: T) -> Self;
}

/// Helper trait for equality comparison
trait AsEqualityComparable<T> {
    fn equals<U: PartialEq<T>>(&self, expected: &U) -> bool;
}

// Implementation for T comparing with anything that can be compared with T
impl<T: PartialEq> AsEqualityComparable<T> for T {
    fn equals<U: PartialEq<T>>(&self, expected: &U) -> bool {
        expected == self
    }
}

// Implementation for &T comparing with anything that can be compared with T
impl<T: PartialEq> AsEqualityComparable<T> for &T {
    fn equals<U: PartialEq<T>>(&self, expected: &U) -> bool {
        expected == *self
    }
}

// Generic implementation for both T and &T expected values
impl<V, T> EqualityMatchers<T> for Assertion<V>
where
    T: Debug + PartialEq + Clone,
    V: AsEqualityComparable<T> + Debug + Clone,
{
    fn to_equal(self, expected: T) -> Self {
        return self.to_equal_value(expected);
    }

    fn to_equal_value(self, expected: T) -> Self {
        let result = self.value.equals(&expected);
        let sentence = AssertionSentence::new("be", format!("equal to {:?}", expected))
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }
}

// We no longer need a separate implementation for &T, since the generic implementation handles it

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_equality() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // Test with integers
        expect!(42).to_equal(42);
        expect!(42).not().to_equal(100);

        // Test with strings
        expect!("hello").to_equal("hello");
        expect!("hello").not().to_equal("world");

        // Test with floating point
        expect!(3.14).to_equal(3.14);
        expect!(3.14).not().to_equal(2.71);

        // Test with boolean
        expect!(true).to_equal(true);
        expect!(true).not().to_equal(false);
    }

    #[test]
    #[should_panic(expected = "be equal to")]
    fn test_equality_fails() {
        // This should fail because 42 != 43
        let _assertion = expect!(42).to_equal(43);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "not be equal to")]
    fn test_equality_not_fails() {
        // This should fail because !(42 != 42)
        let _assertion = expect!(42).not().to_equal(42);
        std::hint::black_box(_assertion);
    }
}
