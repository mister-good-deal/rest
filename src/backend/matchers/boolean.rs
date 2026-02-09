use crate::backend::Assertion;
use crate::backend::assertions::sentence::AssertionSentence;
use std::fmt::Debug;

pub trait BooleanMatchers {
    fn to_be_true(self) -> Self;
    fn to_be_false(self) -> Self;
}

/// Helper trait for boolean-like types
trait AsBoolean {
    fn is_true(&self) -> bool;
    fn is_false(&self) -> bool;
}

// Implementation for bool
impl AsBoolean for bool {
    fn is_true(&self) -> bool {
        *self
    }

    fn is_false(&self) -> bool {
        !*self
    }
}

// Implementation for &bool
impl AsBoolean for &bool {
    fn is_true(&self) -> bool {
        **self
    }

    fn is_false(&self) -> bool {
        !**self
    }
}

// Single implementation for any type that implements AsBoolean
impl<V> BooleanMatchers for Assertion<V>
where
    V: AsBoolean + Debug + Clone,
{
    fn to_be_true(self) -> Self {
        let result = self.value.is_true();
        let sentence = AssertionSentence::new("be", "true")
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_be_false(self) -> Self {
        let result = self.value.is_false();
        let sentence = AssertionSentence::new("be", "false")
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_boolean_true() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // This should pass
        expect!(true).to_be_true();
        expect!(false).not().to_be_true();
    }

    #[test]
    #[should_panic(expected = "not be true")]
    fn test_not_true_fails() {
        // This will evaluate and panic when the Assertion is dropped
        let _assertion = expect!(true).not().to_be_true();
        // Force the value to be dropped at the end of the function
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "be true")]
    fn test_false_to_be_true_fails() {
        // This will evaluate and panic when the Assertion is dropped
        let _assertion = expect!(false).to_be_true();
        // Force the value to be dropped at the end of the function
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_boolean_false() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // This should pass
        expect!(false).to_be_false();
        expect!(true).not().to_be_false();
    }

    #[test]
    #[should_panic(expected = "not be false")]
    fn test_not_false_fails() {
        // This will evaluate and panic when the Assertion is dropped
        let _assertion = expect!(false).not().to_be_false();
        // Force the value to be dropped at the end of the function
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "be false")]
    fn test_true_to_be_false_fails() {
        // This will evaluate and panic when the Assertion is dropped
        let _assertion = expect!(true).to_be_false();
        // Force the value to be dropped at the end of the function
        std::hint::black_box(_assertion);
    }
}
