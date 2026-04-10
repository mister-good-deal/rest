use crate::backend::assertions::sentence::AssertionSentence;
use std::any::Any;

/// Represents the result of catching a panic, with fluent assertion methods.
pub struct PanicAssertion {
    result: Result<(), Box<dyn Any + Send>>,
    expr_str: &'static str,
    negated: bool,
}

impl PanicAssertion {
    /// Creates a new panic assertion from a `catch_unwind` result.
    pub fn new(result: Result<(), Box<dyn Any + Send>>, expr_str: &'static str) -> Self {
        return Self { result, expr_str, negated: false };
    }

    /// Negates the next assertion.
    #[allow(clippy::should_implement_trait)]
    pub fn not(mut self) -> Self {
        self.negated = !self.negated;
        return self;
    }

    /// Asserts that the closure panicked (or did not panic when negated).
    pub fn to_panic(self) -> Self {
        let panicked = self.result.is_err();
        let passed = if self.negated { !panicked } else { panicked };

        if !passed {
            let sentence = if self.negated {
                AssertionSentence::new("panic", "").with_negation(true).with_actual("panicked")
            } else {
                AssertionSentence::new("panic", "").with_actual("did not panic")
            };

            panic!("{}", sentence.format_with_actual());
        }

        return Self { result: self.result, expr_str: self.expr_str, negated: false };
    }

    /// Asserts that the closure panicked with a message exactly matching `expected`.
    pub fn to_have_message(self, expected: &str) -> Self {
        self.assert_panicked();

        let message = self.extract_message();
        let matches = message.as_deref() == Some(expected);
        let passed = if self.negated { !matches } else { matches };

        if !passed {
            let actual_display = message.as_deref().map(|m| format!("\"{}\"", m)).unwrap_or_else(|| "non-string panic payload".to_string());
            let sentence = AssertionSentence::new("have", format!("panic message \"{}\"", expected))
                .with_negation(self.negated)
                .with_actual(actual_display);

            panic!("{}", sentence.format_with_actual());
        }

        return Self { result: self.result, expr_str: self.expr_str, negated: false };
    }

    /// Asserts that the panic message contains `expected` as a substring.
    pub fn to_contain_message(self, expected: &str) -> Self {
        self.assert_panicked();

        let message = self.extract_message();
        let contains = message.as_deref().map(|m| m.contains(expected)).unwrap_or(false);
        let passed = if self.negated { !contains } else { contains };

        if !passed {
            let actual_display = message.as_deref().map(|m| format!("\"{}\"", m)).unwrap_or_else(|| "non-string panic payload".to_string());
            let sentence = AssertionSentence::new("contain", format!("panic message \"{}\"", expected))
                .with_negation(self.negated)
                .with_actual(actual_display);

            panic!("{}", sentence.format_with_actual());
        }

        return Self { result: self.result, expr_str: self.expr_str, negated: false };
    }

    /// Helper: assert that a panic actually occurred (used by message matchers).
    fn assert_panicked(&self) {
        if self.result.is_ok() {
            panic!("expected closure to panic, but it did not panic");
        }
    }

    /// Helper: extract the panic message as a String if possible.
    fn extract_message(&self) -> Option<String> {
        if let Err(ref err) = self.result {
            return extract_panic_message(err);
        }

        return None;
    }
}

/// Extracts a string message from a panic payload.
fn extract_panic_message(err: &Box<dyn Any + Send>) -> Option<String> {
    if let Some(s) = err.downcast_ref::<String>() {
        return Some(s.clone());
    } else if let Some(s) = err.downcast_ref::<&str>() {
        return Some(s.to_string());
    }

    return None;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    fn panicking_function() {
        panic!("something went wrong");
    }

    fn panicking_with_format() {
        panic!("division by zero: {} / {}", 1, 0);
    }

    fn safe_function() -> i32 {
        return 42;
    }

    #[test]
    fn test_expect_panic_basic() {
        expect_panic!(|| {
            panicking_function();
        })
        .to_panic();
    }

    #[test]
    fn test_expect_panic_inline() {
        expect_panic!(|| panic!("boom")).to_panic();
    }

    #[test]
    fn test_expect_panic_to_have_message() {
        expect_panic!(|| {
            panicking_function();
        })
        .to_have_message("something went wrong");
    }

    #[test]
    fn test_expect_panic_to_have_message_formatted() {
        expect_panic!(|| {
            panicking_with_format();
        })
        .to_have_message("division by zero: 1 / 0");
    }

    #[test]
    fn test_expect_panic_to_contain_message() {
        expect_panic!(|| {
            panicking_function();
        })
        .to_contain_message("went wrong");
    }

    #[test]
    fn test_expect_panic_to_contain_message_substring() {
        expect_panic!(|| {
            panicking_with_format();
        })
        .to_contain_message("division");
    }

    #[test]
    fn test_expect_no_panic_basic() {
        expect_no_panic!(|| {
            safe_function();
        });
    }

    #[test]
    fn test_expect_panic_not_to_panic() {
        expect_panic!(|| {
            safe_function();
        })
        .not()
        .to_panic();
    }

    #[test]
    fn test_expect_panic_not_to_have_message() {
        expect_panic!(|| {
            panicking_function();
        })
        .not()
        .to_have_message("some other message");
    }

    #[test]
    fn test_expect_panic_not_to_contain_message() {
        expect_panic!(|| {
            panicking_function();
        })
        .not()
        .to_contain_message("division");
    }

    #[test]
    #[should_panic(expected = "did not panic")]
    fn test_expect_panic_on_safe_code_fails() {
        expect_panic!(|| {
            safe_function();
        })
        .to_panic();
    }

    #[test]
    #[should_panic(expected = "panicked")]
    fn test_expect_no_panic_on_panicking_code_fails() {
        expect_no_panic!(|| {
            panicking_function();
        });
    }

    #[test]
    #[should_panic(expected = "have panic message")]
    fn test_expect_panic_wrong_message_fails() {
        expect_panic!(|| {
            panicking_function();
        })
        .to_have_message("wrong message");
    }

    #[test]
    #[should_panic(expected = "contain panic message")]
    fn test_expect_panic_wrong_substring_fails() {
        expect_panic!(|| {
            panicking_function();
        })
        .to_contain_message("zzz_not_found");
    }

    #[test]
    #[should_panic(expected = "expected closure to panic")]
    fn test_to_have_message_on_safe_code_fails() {
        expect_panic!(|| {
            safe_function();
        })
        .to_have_message("anything");
    }

    #[test]
    #[should_panic(expected = "expected closure to panic")]
    fn test_to_contain_message_on_safe_code_fails() {
        expect_panic!(|| {
            safe_function();
        })
        .to_contain_message("anything");
    }

    #[test]
    fn test_panic_with_non_string_payload() {
        expect_panic!(|| {
            std::panic::panic_any(42i32);
        })
        .to_panic();
    }

    #[test]
    #[should_panic(expected = "non-string panic payload")]
    fn test_panic_with_non_string_payload_message_fails() {
        expect_panic!(|| {
            std::panic::panic_any(42i32);
        })
        .to_have_message("42");
    }

    #[test]
    fn test_panic_with_empty_message() {
        expect_panic!(|| {
            panic!("");
        })
        .to_have_message("");
    }

    #[test]
    fn test_panic_with_string_type() {
        expect_panic!(|| {
            std::panic::panic_any(String::from("owned panic"));
        })
        .to_have_message("owned panic");
    }
}
