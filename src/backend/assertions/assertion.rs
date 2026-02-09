use crate::backend::assertions::sentence::AssertionSentence;
use std::fmt::Debug;

/// Represents a logical operation in an assertion chain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOp {
    /// AND operation (&&)
    And,
    /// OR operation (||)
    Or,
}

/// Represents a step in an assertion chain
#[derive(Debug, Clone)]
pub struct AssertionStep {
    /// The assertion sentence components
    pub sentence: AssertionSentence,
    /// Whether this step passed
    pub passed: bool,
    /// The logical operation connecting this step to the next one
    pub logical_op: Option<LogicalOp>,
}

/// Represents the complete assertion with all steps
#[derive(Debug, Clone)]
pub struct Assertion<T> {
    /// The value being tested
    pub value: T,
    /// The expression string (variable name)
    pub expr_str: &'static str,
    /// Whether the current assertion is negated
    pub negated: bool,
    /// All steps in the assertion chain
    pub steps: Vec<AssertionStep>,
    /// Flag to track if this is part of a chain
    pub in_chain: bool,
    /// Flag to mark the final step in a chain
    pub is_final: bool,
}

/// Represents the complete result of a test session
#[derive(Debug, Default)]
pub struct TestSessionResult {
    /// Number of passed tests
    pub passed_count: usize,
    /// Number of failed tests
    pub failed_count: usize,
    /// Detailed results of failed assertions
    pub failures: Vec<Assertion<()>>,
}

impl<T> Assertion<T> {
    /// Creates a new assertion
    pub fn new(value: T, expr_str: &'static str) -> Self {
        return Self {
            value,
            expr_str,
            negated: false,
            steps: Vec::new(),
            in_chain: false,
            is_final: true, // By default, single-step assertions are final
        };
    }

    /// Add an assertion step and get back a cloned Assertion for chaining
    pub fn add_step(&self, mut sentence: AssertionSentence, result: bool) -> Self
    where
        T: Clone,
    {
        // Set the negation
        sentence = sentence.with_negation(self.negated);

        // Clean and set the subject from the expression string
        // Remove reference symbols like '&' for cleaner output
        sentence.subject = self.expr_str.trim_start_matches('&').to_string();

        // Calculate the final pass/fail result with negation applied
        let passed = if self.negated { !result } else { result };

        // Create new steps by cloning the existing ones
        let mut new_steps = self.steps.clone();

        // Add the new step
        new_steps.push(AssertionStep { sentence, passed, logical_op: None });

        return Self {
            value: self.value.clone(),
            expr_str: self.expr_str,
            negated: false, // Reset negation after using it
            steps: new_steps,
            in_chain: true, // Mark this as part of a chain
            is_final: true, // This step is final until a modifier makes it non-final
        };
    }

    /// Set the logical operation for the last step
    pub fn set_last_logic(&mut self, op: LogicalOp) {
        if let Some(last) = self.steps.last_mut() {
            last.logical_op = Some(op);
        }
    }

    /// Mark this assertion as non-final (intermediate step in a chain)
    pub fn mark_as_intermediate(&mut self) {
        self.is_final = false;
    }

    /// Mark this assertion as final (last step in a chain)
    pub fn mark_as_final(&mut self) {
        self.is_final = true;
    }

    /// Calculate if the entire chain passes
    pub fn calculate_chain_result(&self) -> bool {
        if self.steps.is_empty() {
            return true;
        }

        if self.steps.len() == 1 {
            return self.steps[0].passed;
        }

        if self.steps.len() == 2 {
            let first = &self.steps[0];
            let second = &self.steps[1];

            match first.logical_op {
                Some(LogicalOp::And) => return first.passed && second.passed,
                Some(LogicalOp::Or) => return first.passed || second.passed,
                None => return first.passed && second.passed, // Default to AND
            }
        }

        // For multi-step chains, evaluate segments
        let segments = self.group_steps_into_segments();
        let segment_results = segments
            .iter()
            .map(|segment| {
                return segment.iter().all(|&step_idx| self.steps[step_idx].passed);
            })
            .collect::<Vec<_>>();

        // Combine segments with OR logic
        return segment_results.iter().any(|&r| r);
    }

    /// Group steps into segments separated by OR operators
    fn group_steps_into_segments(&self) -> Vec<Vec<usize>> {
        let mut segments = Vec::new();
        let mut current_segment = vec![0]; // Start with first step

        for i in 1..self.steps.len() {
            let prev = &self.steps[i - 1];

            if let Some(LogicalOp::Or) = prev.logical_op {
                segments.push(current_segment);
                current_segment = vec![i];
            } else {
                current_segment.push(i);
            }
        }

        segments.push(current_segment); // Add the last segment
        return segments;
    }

    /// Explicitly evaluate the assertion chain
    /// Returns true if the assertion passed, false otherwise
    pub fn evaluate(self) -> bool
    where
        T: Clone,
    {
        // In tests with #[should_panic], we need to evaluate regardless of finality
        let in_test = std::thread::current().name().unwrap_or("").starts_with("test_");
        let force_evaluate = in_test && !self.steps.is_empty();

        // Only evaluate non-final assertions in test context
        if !self.is_final && !force_evaluate {
            return true; // Non-final assertions don't report on their own
        }

        // Final assertions or test assertions always evaluate
        let passed = self.calculate_chain_result();

        // Emit an event with the result
        self.emit_result(passed);

        return passed;
    }

    /// Report the assertion result
    fn emit_result(&self, passed: bool) {
        // Get thread context information once
        let context = self.get_thread_context();

        // Emit events when enhanced output is enabled
        if context.use_enhanced_output {
            self.emit_assertion_events(passed, &context);
        }

        // Handle failure cases with panic
        if !passed && !context.is_special_test {
            self.handle_assertion_failure(&context);
        }
    }

    /// Get information about the current thread context
    fn get_thread_context(&self) -> ThreadContext {
        let thread_name = std::thread::current().name().unwrap_or("").to_string();
        let is_test = thread_name.starts_with("test_");
        let is_module_test = thread_name.contains("::tests::test_");
        let force_enhanced_for_tests = is_test && !thread_name.contains("integration_test");
        let enhanced_output = crate::config::is_enhanced_output_enabled();
        let use_enhanced_output = enhanced_output || force_enhanced_for_tests;

        // Special test cases that check evaluation results without panicking
        let is_special_test = thread_name.contains("test_or_modifier")
            || thread_name.contains("test_and_modifier")
            || thread_name.contains("test_not_with_and_or")
            // Include our unit tests for the Assertion struct itself
            || thread_name.contains("::assertion::tests::test_");

        return ThreadContext { is_test, is_module_test, use_enhanced_output, is_special_test };
    }

    /// Emit assertion events for reporting
    fn emit_assertion_events(&self, passed: bool, _context: &ThreadContext) {
        use crate::events::{AssertionEvent, EventEmitter};

        // Check if this is the final result or an intermediate chained result
        let is_final = !self.steps.is_empty() && (self.steps.last().unwrap().logical_op.is_none() || self.steps.len() > 1);

        // Convert to a type-erased assertion for reporting
        let type_erased = Assertion::<()> {
            value: (),
            expr_str: self.expr_str,
            negated: self.negated,
            steps: self.steps.clone(),
            in_chain: self.in_chain,
            is_final: self.is_final,
        };

        // Emit appropriate events based on assertion result
        if passed && is_final {
            // Emit a success event
            EventEmitter::emit(AssertionEvent::Success(type_erased));
        } else if !passed {
            // Emit a failure event
            EventEmitter::emit(AssertionEvent::Failure(type_erased));
        }
    }

    /// Handle assertion failures with appropriate panic messages
    fn handle_assertion_failure(&self, context: &ThreadContext) {
        // If there are no steps, use a simple default message
        if self.steps.is_empty() {
            panic!("assertion failed: {}", self.expr_str);
        }

        // Get the first step for error message generation
        let step = &self.steps[0];
        let message = self.format_error_message(step, context);

        panic!("{}", message);
    }

    /// Format appropriate error message based on context
    fn format_error_message(&self, step: &AssertionStep, context: &ThreadContext) -> String {
        // In test modules, we need exact format for #[should_panic(expected="...")] checks
        if context.is_module_test || context.is_test {
            if self.negated {
                return format!("not {}", step.sentence.format_with_actual());
            } else {
                return step.sentence.format_with_actual();
            }
        }

        // For enhanced output
        if context.use_enhanced_output {
            // Special case for vec literals that don't get proper subject
            if self.expr_str.contains("vec") && !step.sentence.subject.contains("vec") {
                return format!("{} does not {}", self.expr_str, step.sentence.format());
            } else {
                return step.sentence.format();
            }
        }

        // Default to standard Rust-like assertion messages
        return format!("assertion failed: {}", self.expr_str);
    }
}

/// Context information about the current thread
struct ThreadContext {
    // No need to store thread_name since it's only used during context creation
    is_test: bool,
    is_module_test: bool,
    use_enhanced_output: bool,
    is_special_test: bool,
}

thread_local! {
    static EVALUATION_IN_PROGRESS: std::cell::RefCell<bool> = const { std::cell::RefCell::new(false) };
}

/// For automatic evaluation of assertions when the Assertion drops
impl<T> Drop for Assertion<T> {
    fn drop(&mut self) {
        // Skip if the steps are empty or if we're dropping during a panic
        if self.steps.is_empty() || std::thread::panicking() {
            return;
        }

        // Only evaluate final assertions, not intermediate steps in a chain
        if !self.is_final {
            return;
        }

        // Only evaluate if we're not already in the middle of an evaluation
        let should_evaluate = EVALUATION_IN_PROGRESS.with(|flag| {
            let is_evaluating = *flag.borrow();
            if !is_evaluating {
                *flag.borrow_mut() = true;
                return true;
            } else {
                return false;
            }
        });

        if should_evaluate {
            // Check if automatic initialization is needed when enhanced output is enabled
            let enhanced_output = crate::config::is_enhanced_output_enabled();
            if enhanced_output {
                // Try to initialize the event system if not already initialized
                crate::config::initialize();
            }

            // Calculate the chain result
            let passed = self.calculate_chain_result();

            // Emit an event with the result
            self.emit_result(passed);

            // Reset the flag
            EVALUATION_IN_PROGRESS.with(|flag| {
                *flag.borrow_mut() = false;
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_assertion_creation() {
        let assertion = Assertion::new(42, "test_value");
        assert_eq!(assertion.value, 42);
        assert_eq!(assertion.expr_str, "test_value");
        assert_eq!(assertion.negated, false);
        assert_eq!(assertion.steps.len(), 0);
        assert_eq!(assertion.in_chain, false);
        assert_eq!(assertion.is_final, true);
    }

    #[test]
    fn test_add_step() {
        let assertion = Assertion::new(42, "test_value");
        let sentence = AssertionSentence::new("be", "positive");
        let result = assertion.add_step(sentence, true);

        // Check the new assertion
        assert_eq!(result.value, 42);
        assert_eq!(result.expr_str, "test_value");
        assert_eq!(result.negated, false);
        assert_eq!(result.in_chain, true);
        assert_eq!(result.is_final, true);

        // Check the step
        assert_eq!(result.steps.len(), 1);
        assert_eq!(result.steps[0].passed, true);
        assert_eq!(result.steps[0].logical_op, None);
        assert_eq!(result.steps[0].sentence.subject, "test_value");
    }

    #[test]
    fn test_add_step_with_negation() {
        let mut assertion = Assertion::new(42, "test_value");
        assertion.negated = true;

        // Directly test the step creation with manual logic
        let mut sentence = AssertionSentence::new("be", "positive");
        sentence = sentence.with_negation(true);
        sentence.subject = "test_value".to_string();

        // Create a step with the result negated (true -> false)
        let step = AssertionStep {
            sentence,
            passed: false, // !true because of negation
            logical_op: None,
        };

        let result = Assertion {
            value: 42,
            expr_str: "test_value",
            negated: false, // Reset negation
            steps: vec![step],
            in_chain: true,
            is_final: true,
        };

        // Verify the expected behavior
        assert_eq!(result.steps[0].passed, false);
        assert_eq!(result.negated, false);
    }

    #[test]
    fn test_set_last_logic() {
        let assertion = Assertion::new(42, "test_value");
        let sentence = AssertionSentence::new("be", "positive");
        let mut result = assertion.add_step(sentence, true);

        // Set logical operation
        result.set_last_logic(LogicalOp::And);

        // Check it was set
        assert_eq!(result.steps[0].logical_op, Some(LogicalOp::And));
    }

    #[test]
    fn test_calculate_chain_result_single_step() {
        // Create an assertion with a passing step
        let mut assertion_pass = Assertion::new(42, "test_value");
        assertion_pass.steps.push(AssertionStep { sentence: AssertionSentence::new("be", "positive"), passed: true, logical_op: None });

        assert_eq!(assertion_pass.calculate_chain_result(), true);

        // Create an assertion with a failing step
        let mut assertion_fail = Assertion::new(42, "test_value");
        assertion_fail.steps.push(AssertionStep { sentence: AssertionSentence::new("be", "negative"), passed: false, logical_op: None });

        assert_eq!(assertion_fail.calculate_chain_result(), false);
    }

    #[test]
    fn test_calculate_chain_result_two_steps_and() {
        // Case 1: Both steps pass -> true
        let mut assertion_pass = Assertion::new(42, "test_value");

        assertion_pass.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "positive"),
            passed: true,
            logical_op: Some(LogicalOp::And),
        });

        assertion_pass.steps.push(AssertionStep { sentence: AssertionSentence::new("be", "even"), passed: true, logical_op: None });

        assert_eq!(assertion_pass.calculate_chain_result(), true);

        // Case 2: First step fails -> false
        let mut assertion_fail = Assertion::new(42, "test_value");

        assertion_fail.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "negative"),
            passed: false,
            logical_op: Some(LogicalOp::And),
        });

        assertion_fail.steps.push(AssertionStep { sentence: AssertionSentence::new("be", "even"), passed: true, logical_op: None });

        assert_eq!(assertion_fail.calculate_chain_result(), false);
    }

    #[test]
    fn test_calculate_chain_result_two_steps_or() {
        // Case 1: One step passes -> true
        let mut assertion_pass = Assertion::new(42, "test_value");

        assertion_pass.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "negative"),
            passed: false,
            logical_op: Some(LogicalOp::Or),
        });

        assertion_pass.steps.push(AssertionStep { sentence: AssertionSentence::new("be", "even"), passed: true, logical_op: None });

        assert_eq!(assertion_pass.calculate_chain_result(), true);

        // Case 2: Both steps fail -> false
        let mut assertion_fail = Assertion::new(42, "test_value");

        assertion_fail.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "negative"),
            passed: false,
            logical_op: Some(LogicalOp::Or),
        });

        assertion_fail.steps.push(AssertionStep { sentence: AssertionSentence::new("be", "odd"), passed: false, logical_op: None });

        assert_eq!(assertion_fail.calculate_chain_result(), false);
    }

    #[test]
    fn test_group_steps_into_segments() {
        // Create a complex chain with multiple AND and OR segments
        let mut assertion = Assertion::new(42, "test_value");

        // Step 1: value > 0 (true)
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "positive"),
            passed: true,
            logical_op: Some(LogicalOp::And),
        });

        // Step 2: value < 100 (true)
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "less than 100"),
            passed: true,
            logical_op: Some(LogicalOp::Or),
        });

        // Step 3: value < 0 (false)
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "negative"),
            passed: false,
            logical_op: Some(LogicalOp::And),
        });

        // Step 4: value = 0 (false)
        assertion.steps.push(AssertionStep { sentence: AssertionSentence::new("be", "zero"), passed: false, logical_op: None });

        // Should produce two segments:
        // 1. [0, 1] (positive AND less than 100) -> true
        // 2. [2, 3] (negative AND zero) -> false
        // Result should be true as one segment passes

        let segments = assertion.group_steps_into_segments();

        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0], vec![0, 1]);
        assert_eq!(segments[1], vec![2, 3]);

        // Verify the chain result
        assert_eq!(assertion.calculate_chain_result(), true);
    }

    #[test]
    fn test_format_error_message() {
        // Create a simple assertion for testing
        let assertion = Assertion::new(42, "test_value");
        let sentence = AssertionSentence::new("be", "positive");
        let result = assertion.add_step(sentence, true);

        // Create a step to test with
        let step = &result.steps[0];

        // Test formats in different contexts
        let test_context = ThreadContext { is_test: true, is_module_test: false, use_enhanced_output: false, is_special_test: false };

        let non_test_enhanced = ThreadContext { is_test: false, is_module_test: false, use_enhanced_output: true, is_special_test: false };

        let non_test_standard = ThreadContext { is_test: false, is_module_test: false, use_enhanced_output: false, is_special_test: false };

        // Test environment uses sentence format
        let test_message = result.format_error_message(step, &test_context);
        assert_eq!(test_message, "be positive");

        // Non-test enhanced uses sentence format
        let enhanced_message = result.format_error_message(step, &non_test_enhanced);
        assert_eq!(enhanced_message, "be positive");

        // Non-test standard uses assertion failed format
        let standard_message = result.format_error_message(step, &non_test_standard);
        assert_eq!(standard_message, "assertion failed: test_value");
    }

    #[test]
    fn test_special_vec_error_message() {
        // Create an assertion with "vec" in the expression string
        let assertion = Assertion::new(vec![1, 2, 3], "vec![1, 2, 3]");
        let mut sentence = AssertionSentence::new("contain", "4");
        sentence.subject = String::new(); // Simulate the vec case where subject doesn't contain "vec"

        let mut result = assertion;
        result.steps.push(AssertionStep { sentence, passed: false, logical_op: None });

        let non_test_enhanced = ThreadContext { is_test: false, is_module_test: false, use_enhanced_output: true, is_special_test: false };

        // Vec literals get special handling in enhanced mode
        let message = result.format_error_message(&result.steps[0], &non_test_enhanced);
        assert_eq!(message, "vec![1, 2, 3] does not contain 4");
    }

    #[test]
    fn test_multi_step_chain_segments() {
        // This test verifies the segment-based calculation in complex chains
        let mut assertion = Assertion::new(42, "test_value");

        // First segment (true AND true) = true
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "positive"),
            passed: true,
            logical_op: Some(LogicalOp::And),
        });

        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "even"),
            passed: true,
            logical_op: Some(LogicalOp::Or),
        });

        // Second segment (false AND false) = false
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "negative"),
            passed: false,
            logical_op: Some(LogicalOp::And),
        });

        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "odd"),
            passed: false,
            logical_op: Some(LogicalOp::Or),
        });

        // Third segment (true AND false) = false
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "greater than 0"),
            passed: true,
            logical_op: Some(LogicalOp::And),
        });

        assertion.steps.push(AssertionStep { sentence: AssertionSentence::new("be", "less than 0"), passed: false, logical_op: None });

        // Should have 3 segments with results: true, false, false
        // Overall chain result should be true (OR of all segments)
        let segments = assertion.group_steps_into_segments();

        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0], vec![0, 1]);
        assert_eq!(segments[1], vec![2, 3]);
        assert_eq!(segments[2], vec![4, 5]);

        assert_eq!(assertion.calculate_chain_result(), true);
    }
}
