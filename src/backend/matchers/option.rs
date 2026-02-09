use crate::backend::Assertion;
use crate::backend::assertions::sentence::AssertionSentence;
use std::fmt::Debug;

/// Trait for Option<T> assertions
pub trait OptionMatchers<T: Debug> {
    fn to_be_some(self) -> Self;
    fn to_be_none(self) -> Self;
    fn to_contain(self, expected: &T) -> Self
    where
        T: PartialEq;
}

/// Helper trait for Optiony types
trait AsOption {
    type Item: Debug;

    fn is_some_option(&self) -> bool;
    fn is_none_option(&self) -> bool;
    fn contains_item<U>(&self, expected: &U) -> bool
    where
        U: PartialEq<Self::Item>;
}

// Implementation for Option<T>
impl<T: Debug + PartialEq> AsOption for Option<T> {
    type Item = T;

    fn is_some_option(&self) -> bool {
        self.is_some()
    }

    fn is_none_option(&self) -> bool {
        self.is_none()
    }

    fn contains_item<U>(&self, expected: &U) -> bool
    where
        U: PartialEq<Self::Item>,
    {
        match self {
            Some(actual) => expected == actual,
            None => false,
        }
    }
}

// Implementation for &Option<T>
impl<T: Debug + PartialEq> AsOption for &Option<T> {
    type Item = T;

    fn is_some_option(&self) -> bool {
        self.is_some()
    }

    fn is_none_option(&self) -> bool {
        self.is_none()
    }

    fn contains_item<U>(&self, expected: &U) -> bool
    where
        U: PartialEq<Self::Item>,
    {
        match self {
            Some(actual) => expected == actual,
            None => false,
        }
    }
}

// Single implementation of OptionMatchers for any type that implements AsOption
impl<T, V> OptionMatchers<T> for Assertion<V>
where
    T: Debug + Clone + PartialEq,
    V: AsOption<Item = T> + Debug + Clone,
{
    fn to_be_some(self) -> Self {
        let result = self.value.is_some_option();
        let sentence = AssertionSentence::new("be", "some")
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_be_none(self) -> Self {
        let result = self.value.is_none_option();
        let sentence = AssertionSentence::new("be", "none")
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_contain(self, expected: &T) -> Self
    where
        T: PartialEq,
    {
        let result = self.value.contains_item(expected);
        let sentence = AssertionSentence::new("contain", format!("{:?}", expected))
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_option_to_be_some() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let some_value: Option<i32> = Some(42);
        let none_value: Option<i32> = None;

        // These should pass
        expect!(some_value).to_be_some();
        expect!(none_value).to_be_none();
        expect!(none_value).not().to_be_some();
        expect!(some_value).not().to_be_none();
    }

    #[test]
    #[should_panic(expected = "be some")]
    fn test_none_to_be_some_fails() {
        let value: Option<i32> = None;
        let _assertion = expect!(value).to_be_some();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "be none")]
    fn test_some_to_be_none_fails() {
        let value: Option<i32> = Some(42);
        let _assertion = expect!(value).to_be_none();
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_option_to_contain() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let value: Option<i32> = Some(42);

        // These should pass
        expect!(value).to_contain(&42);
        expect!(value).not().to_contain(&43);

        let none_value: Option<i32> = None;
        expect!(none_value).not().to_contain(&42);
    }

    #[test]
    #[should_panic(expected = "contain")]
    fn test_wrong_value_not_fails() {
        let value: Option<i32> = Some(42);
        let _assertion = expect!(value).not().to_contain(&42);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "contain 43")]
    fn test_missing_value_fails() {
        let value: Option<i32> = Some(42);
        let _assertion = expect!(value).to_contain(&43);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "contain 42")]
    fn test_none_value_fails() {
        let value: Option<i32> = None;
        let _assertion = expect!(value).to_contain(&42);
        std::hint::black_box(_assertion);
    }
}
