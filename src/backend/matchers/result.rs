use crate::backend::Assertion;
use crate::backend::assertions::sentence::AssertionSentence;
use std::fmt::Debug;

/// Trait for Result<T, E> assertions
pub trait ResultMatchers<T: Debug, E: Debug> {
    fn to_be_ok(self) -> Self;
    fn to_be_err(self) -> Self;
    fn to_contain_ok<U: PartialEq<T> + Debug>(self, expected: &U) -> Self;
    fn to_contain_err<U: PartialEq<E> + Debug>(self, expected: &U) -> Self;
}

/// Helper trait for Result-like types
trait AsResult<T: Debug + Clone, E: Debug + Clone> {
    fn is_ok_result(&self) -> bool;
    fn is_err_result(&self) -> bool;
    fn contains_ok<U: PartialEq<T> + Debug>(&self, expected: &U) -> bool;
    fn contains_err<U: PartialEq<E> + Debug>(&self, expected: &U) -> bool;
}

// Implementation for Result<T, E>
impl<T: Debug + Clone, E: Debug + Clone> AsResult<T, E> for Result<T, E> {
    fn is_ok_result(&self) -> bool {
        self.is_ok()
    }

    fn is_err_result(&self) -> bool {
        self.is_err()
    }

    fn contains_ok<U: PartialEq<T> + Debug>(&self, expected: &U) -> bool {
        match self {
            Ok(actual) => expected == actual,
            Err(_) => false,
        }
    }

    fn contains_err<U: PartialEq<E> + Debug>(&self, expected: &U) -> bool {
        match self {
            Ok(_) => false,
            Err(actual) => expected == actual,
        }
    }
}

// Implementation for &Result<T, E>
impl<T: Debug + Clone, E: Debug + Clone> AsResult<T, E> for &Result<T, E> {
    fn is_ok_result(&self) -> bool {
        self.is_ok()
    }

    fn is_err_result(&self) -> bool {
        self.is_err()
    }

    fn contains_ok<U: PartialEq<T> + Debug>(&self, expected: &U) -> bool {
        match self {
            Ok(actual) => expected == actual,
            Err(_) => false,
        }
    }

    fn contains_err<U: PartialEq<E> + Debug>(&self, expected: &U) -> bool {
        match self {
            Ok(_) => false,
            Err(actual) => expected == actual,
        }
    }
}

// Single implementation for any type that implements AsResult
impl<V, T, E> ResultMatchers<T, E> for Assertion<V>
where
    T: Debug + Clone,
    E: Debug + Clone,
    V: AsResult<T, E> + Debug + Clone,
{
    fn to_be_ok(self) -> Self {
        let result = self.value.is_ok_result();
        let sentence = AssertionSentence::new("be", "ok")
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_be_err(self) -> Self {
        let result = self.value.is_err_result();
        let sentence = AssertionSentence::new("be", "err")
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_contain_ok<U: PartialEq<T> + Debug>(self, expected: &U) -> Self {
        let result = self.value.contains_ok(expected);
        let sentence = AssertionSentence::new("contain", format!("ok value {:?}", expected))
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_contain_err<U: PartialEq<E> + Debug>(self, expected: &U) -> Self {
        let result = self.value.contains_err(expected);
        let sentence = AssertionSentence::new("contain", format!("err value {:?}", expected))
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_result_to_be_ok() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let ok_value: Result<i32, &str> = Ok(42);
        let err_value: Result<i32, &str> = Err("error");

        // These should pass
        expect!(ok_value).to_be_ok();
        expect!(err_value).to_be_err();
        expect!(err_value).not().to_be_ok();
        expect!(ok_value).not().to_be_err();
    }

    #[test]
    #[should_panic(expected = "be ok")]
    fn test_err_to_be_ok_fails() {
        let value: Result<i32, &str> = Err("error");
        let _assertion = expect!(value).to_be_ok();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "be err")]
    fn test_ok_to_be_err_fails() {
        let value: Result<i32, &str> = Ok(42);
        let _assertion = expect!(value).to_be_err();
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_result_contain_values() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let ok_value: Result<i32, &str> = Ok(42);
        let err_value: Result<i32, &str> = Err("error");

        // These should pass
        expect!(ok_value).to_contain_ok(&42);
        expect!(ok_value).not().to_contain_ok(&43);
        expect!(err_value).to_contain_err(&"error");
        expect!(err_value).not().to_contain_err(&"different");
    }

    #[test]
    #[should_panic(expected = "contain ok value")]
    fn test_ok_wrong_value_fails() {
        let value: Result<i32, &str> = Ok(42);
        let _assertion = expect!(value).to_contain_ok(&43);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "not contain ok value")]
    fn test_ok_right_value_not_fails() {
        let value: Result<i32, &str> = Ok(42);
        let _assertion = expect!(value).not().to_contain_ok(&42);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "contain err value")]
    fn test_err_wrong_value_fails() {
        let value: Result<i32, &str> = Err("error");
        let _assertion = expect!(value).to_contain_err(&"different");
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "not contain err value")]
    fn test_err_right_value_not_fails() {
        let value: Result<i32, &str> = Err("error");
        let _assertion = expect!(value).not().to_contain_err(&"error");
        std::hint::black_box(_assertion);
    }
}
