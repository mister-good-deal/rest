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
}
