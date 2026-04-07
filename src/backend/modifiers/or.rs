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
}
