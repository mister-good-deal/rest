use crate::backend::Assertion;
use crate::backend::LogicalOp;

/// AND modifier trait for chaining assertions
pub trait AndModifier<T> {
    /// Creates an AND-chained assertion that allows for multiple assertions on the same value
    /// This provides a fluent API for multiple assertions:
    /// expect(value).to_be_greater_than(5).and().to_be_less_than(10)
    fn and(self) -> Self;
}

impl<T: Clone> AndModifier<T> for Assertion<T> {
    /// Returns a new Assertion with the same value, allowing for chaining assertions
    fn and(self) -> Self {
        // The previous assertion was intermediate (not final)
        let mut result = self;
        result.mark_as_intermediate();

        // Set the logical operator for the last step
        result.set_last_logic(LogicalOp::And);

        return Self {
            value: result.value.clone(),
            expr_str: result.expr_str,
            negated: result.negated,
            steps: result.steps.clone(),
            in_chain: true,  // Always mark as part of a chain
            is_final: false, // This is not the final step - there will be more after 'and()'
            evaluated: false,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_and_modifier() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let value = 42;

        // Simple test that doesn't trigger thread-local issues
        expect!(value).to_be_greater_than(30).and().to_be_less_than(50);
    }

    #[test]
    fn test_and_three_step_chain() {
        crate::Reporter::disable_deduplication();
        crate::Reporter::enable_silent_mode();

        let value = 42;
        expect!(value).to_be_greater_than(10).and().to_be_less_than(100).and().to_be_positive();

        crate::Reporter::disable_silent_mode();
    }

    #[test]
    fn test_and_four_step_chain() {
        crate::Reporter::disable_deduplication();
        crate::Reporter::enable_silent_mode();

        let value = 42;
        expect!(value).to_be_greater_than(0).and().to_be_less_than(100).and().to_be_positive().and().to_be_even();

        crate::Reporter::disable_silent_mode();
    }

    #[test]
    fn test_and_preserves_chain_state() {
        crate::Reporter::disable_deduplication();
        crate::Reporter::enable_silent_mode();

        let value = 42;
        let assertion = expect!(value).to_be_greater_than(10).and();

        // After and(), the assertion should be in a chain
        assert!(assertion.in_chain, "should be marked as in_chain after and()");
        assert!(!assertion.is_final, "should not be final after and()");

        // Prevent drop evaluation
        let mut assertion = assertion;
        assertion.evaluated = true;

        crate::Reporter::disable_silent_mode();
    }
}
