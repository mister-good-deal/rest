use crate::backend::Assertion;
use crate::backend::LogicalOp;

/// OR modifier trait for chaining assertions
pub trait OrModifier<T> {
    /// Creates an OR-chained assertion that allows for multiple assertions on the same value
    /// This provides a fluent API for alternative assertions:
    /// expect(value).to_be_greater_than(100).or().to_be_less_than(0)
    fn or(self) -> Self;
}

impl<T: Clone> OrModifier<T> for Assertion<T> {
    /// Returns a new Assertion with the same value, allowing for OR chaining assertions
    fn or(self) -> Self {
        // The previous assertion was intermediate (not final)
        let mut result = self;
        result.mark_as_intermediate();

        // Set the logical operator for the last step
        result.set_last_logic(LogicalOp::Or);

        return Self {
            value: result.value.clone(),
            expr_str: result.expr_str,
            negated: result.negated,
            steps: result.steps.clone(),
            in_chain: true,  // Always mark as part of a chain
            is_final: false, // This is not the final step - there will be more after 'or()'
            evaluated: false,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_or_modifier() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let value = 42;

        // Simple test that doesn't trigger thread-local issues
        expect!(value).to_be_greater_than(100).or().to_be_less_than(100);
    }

    #[test]
    fn test_or_three_step_chain() {
        crate::Reporter::disable_deduplication();
        crate::Reporter::enable_silent_mode();

        let value = 5;
        // 5 != 3 (fail) OR 5 == 5 (pass) OR 5 != 7 (fail) => pass
        expect!(value).to_equal(3).or().to_equal(5).or().to_equal(7);

        crate::Reporter::disable_silent_mode();
    }

    #[test]
    fn test_or_first_passes_short_circuit() {
        crate::Reporter::disable_deduplication();
        crate::Reporter::enable_silent_mode();

        let value = 10;
        // First condition passes, rest don't matter
        expect!(value).to_be_positive().or().to_equal(999);

        crate::Reporter::disable_silent_mode();
    }

    #[test]
    fn test_or_last_passes() {
        crate::Reporter::disable_deduplication();
        crate::Reporter::enable_silent_mode();

        let value = 42;
        // Only last condition passes
        expect!(value).to_equal(1).or().to_equal(2).or().to_equal(42);

        crate::Reporter::disable_silent_mode();
    }

    #[test]
    fn test_or_preserves_chain_state() {
        crate::Reporter::disable_deduplication();
        crate::Reporter::enable_silent_mode();

        let value = 42;
        let assertion = expect!(value).to_equal(1).or();

        assert!(assertion.in_chain, "should be marked as in_chain after or()");
        assert!(!assertion.is_final, "should not be final after or()");

        let mut assertion = assertion;
        assertion.evaluated = true;

        crate::Reporter::disable_silent_mode();
    }
}
