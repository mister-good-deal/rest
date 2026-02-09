use crate::backend::Assertion;
use crate::backend::assertions::sentence::AssertionSentence;
use regex::Regex;
use std::fmt::Debug;

/// Trait for string assertions
pub trait StringMatchers {
    fn to_be_empty(self) -> Self;
    fn to_have_length(self, expected: usize) -> Self;

    /// Check if the string contains a substring
    fn to_contain(self, substring: &str) -> Self;

    /// Type-specific version of to_contain to avoid trait conflicts
    fn to_contain_substring(self, substring: &str) -> Self;

    fn to_start_with(self, prefix: &str) -> Self;
    fn to_end_with(self, suffix: &str) -> Self;
    fn to_match(self, pattern: &str) -> Self;
}

/// Helper trait for string-like types
trait AsString {
    fn is_empty_string(&self) -> bool;
    fn length_string(&self) -> usize;
    fn contains_substring(&self, substring: &str) -> bool;
    fn starts_with_substring(&self, prefix: &str) -> bool;
    fn ends_with_substring(&self, suffix: &str) -> bool;
    fn matches_pattern(&self, pattern: &str) -> bool;
}

// Implementation for String
impl AsString for String {
    fn is_empty_string(&self) -> bool {
        self.is_empty()
    }

    fn length_string(&self) -> usize {
        self.len()
    }

    fn contains_substring(&self, substring: &str) -> bool {
        self.contains(substring)
    }

    fn starts_with_substring(&self, prefix: &str) -> bool {
        self.starts_with(prefix)
    }

    fn ends_with_substring(&self, suffix: &str) -> bool {
        self.ends_with(suffix)
    }

    fn matches_pattern(&self, pattern: &str) -> bool {
        let re = Regex::new(pattern).unwrap_or_else(|e| {
            panic!("Invalid regex pattern '{}': {}", pattern, e);
        });

        return re.is_match(self);
    }
}

// Implementation for &str
impl AsString for &str {
    fn is_empty_string(&self) -> bool {
        self.is_empty()
    }

    fn length_string(&self) -> usize {
        self.len()
    }

    fn contains_substring(&self, substring: &str) -> bool {
        self.contains(substring)
    }

    fn starts_with_substring(&self, prefix: &str) -> bool {
        self.starts_with(prefix)
    }

    fn ends_with_substring(&self, suffix: &str) -> bool {
        self.ends_with(suffix)
    }

    fn matches_pattern(&self, pattern: &str) -> bool {
        let re = Regex::new(pattern).unwrap_or_else(|e| {
            panic!("Invalid regex pattern '{}': {}", pattern, e);
        });

        return re.is_match(self);
    }
}

// Single implementation for any type that implements AsString
impl<V> StringMatchers for Assertion<V>
where
    V: AsString + Debug + Clone,
{
    fn to_be_empty(self) -> Self {
        let result = self.value.is_empty_string();
        let sentence = AssertionSentence::new("be", "empty")
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_have_length(self, expected: usize) -> Self {
        let actual_length = self.value.length_string();
        let result = actual_length == expected;
        let sentence = AssertionSentence::new("have", format!("length {}", expected))
            .with_actual(format!("{}", actual_length));

        return self.add_step(sentence, result);
    }

    fn to_contain(self, substring: &str) -> Self {
        return self.to_contain_substring(substring);
    }

    fn to_contain_substring(self, substring: &str) -> Self {
        let result = self.value.contains_substring(substring);
        let sentence = AssertionSentence::new("contain", format!("\"{}\"", substring))
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_start_with(self, prefix: &str) -> Self {
        let result = self.value.starts_with_substring(prefix);
        let sentence = AssertionSentence::new("start with", format!("\"{}\"", prefix))
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_end_with(self, suffix: &str) -> Self {
        let result = self.value.ends_with_substring(suffix);
        let sentence = AssertionSentence::new("end with", format!("\"{}\"", suffix))
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }

    fn to_match(self, pattern: &str) -> Self {
        let result = self.value.matches_pattern(pattern);
        let sentence = AssertionSentence::new("match", format!("pattern /{}/", pattern))
            .with_actual(format!("{:?}", self.value));

        return self.add_step(sentence, result);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_string_to_be_empty() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass
        expect!("").to_be_empty();
        expect!("hello").not().to_be_empty();
        expect!(String::new()).to_be_empty();
        expect!(String::from("hello")).not().to_be_empty();
    }

    #[test]
    #[should_panic(expected = "be empty")]
    fn test_non_empty_to_be_empty_fails() {
        let _assertion = expect!("hello").to_be_empty();
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "not be empty")]
    fn test_empty_not_to_be_empty_fails() {
        let _assertion = expect!("").not().to_be_empty();
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_string_to_have_length() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass
        expect!("hello").to_have_length(5);
        expect!("hello").not().to_have_length(4);
        expect!(String::from("hello")).to_have_length(5);
    }

    #[test]
    #[should_panic(expected = "have length")]
    fn test_wrong_length_fails() {
        let _assertion = expect!("hello").to_have_length(4);
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "not have length")]
    fn test_right_length_not_fails() {
        let _assertion = expect!("hello").not().to_have_length(5);
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_string_to_contain() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass
        expect!("hello world").to_contain("hello");
        expect!("hello world").not().to_contain("goodbye");
        expect!(String::from("hello world")).to_contain("world");
    }

    #[test]
    #[should_panic(expected = "not contain")]
    fn test_not_contains_when_it_does_fails() {
        let _assertion = expect!("hello world").not().to_contain("hello");
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "contain")]
    fn test_contains_when_it_doesnt_fails() {
        let _assertion = expect!("hello world").to_contain("goodbye");
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_string_to_start_with() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass
        expect!("hello world").to_start_with("hello");
        expect!("hello world").not().to_start_with("world");
        expect!(String::from("hello world")).to_start_with("hello");
    }

    #[test]
    #[should_panic(expected = "not start with")]
    fn test_not_starts_with_when_it_does_fails() {
        let _assertion = expect!("hello world").not().to_start_with("hello");
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "start with")]
    fn test_starts_with_when_it_doesnt_fails() {
        let _assertion = expect!("hello world").to_start_with("world");
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_string_to_end_with() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // These should pass
        expect!("hello world").to_end_with("world");
        expect!("hello world").not().to_end_with("hello");
        expect!(String::from("hello world")).to_end_with("world");
    }

    #[test]
    #[should_panic(expected = "not end with")]
    fn test_not_ends_with_when_it_does_fails() {
        let _assertion = expect!("hello world").not().to_end_with("world");
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "end with")]
    fn test_ends_with_when_it_doesnt_fails() {
        let _assertion = expect!("hello world").to_end_with("hello");
        std::hint::black_box(_assertion);
    }

    #[test]
    fn test_string_to_match() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        // Plain substring patterns (backward compatible)
        expect!("hello world").to_match("world");
        expect!("hello world").not().to_match("goodbye");
        expect!(String::from("hello world")).to_match("hello");

        // Actual regex patterns
        expect!("hello123").to_match("\\d+");
        expect!("hello123").to_match("^hello\\d+$");
        expect!("hello").not().to_match("\\d+");
        expect!("2024-01-15").to_match("\\d{4}-\\d{2}-\\d{2}");
        expect!("test@example.com").to_match("[a-zA-Z]+@[a-zA-Z]+\\.[a-zA-Z]+");
        expect!(String::from("abc123")).to_match("[a-z]+\\d+");
    }

    #[test]
    #[should_panic(expected = "Invalid regex pattern")]
    fn test_invalid_regex_panics() {
        let _assertion = expect!("hello").to_match("[invalid");
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "not match")]
    fn test_not_matches_when_it_does_fails() {
        let _assertion = expect!("hello world").not().to_match("hello");
        std::hint::black_box(_assertion);
    }

    #[test]
    #[should_panic(expected = "match")]
    fn test_matches_when_it_doesnt_fails() {
        let _assertion = expect!("hello world").to_match("goodbye");
        std::hint::black_box(_assertion);
    }
}
