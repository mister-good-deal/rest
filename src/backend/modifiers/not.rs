use crate::backend::Assertion;

/// Not modifier trait for negating assertions
pub trait NotModifier<T> {
    /// Creates a negated assertion
    fn not(self) -> Self;
}

impl<T: Clone> NotModifier<T> for Assertion<T> {
    /// Creates a negated assertion
    /// This provides a fluent API for negated assertions:
    /// expect(value).not().to_equal(x)
    fn not(self) -> Self {
        return Self {
            value: self.value.clone(),
            expr_str: self.expr_str,
            negated: !self.negated,
            steps: self.steps.clone(),
            in_chain: self.in_chain, // Preserve chain status
            is_final: self.is_final, // Preserve finality status
            evaluated: false,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_not_modifier() {
        // Disable deduplication for tests
        crate::Reporter::disable_deduplication();

        let value = 42;

        // These should pass
        expect!(value).not().to_equal(100);
        expect!(value).not().to_be_less_than(10);

        // Test with chains
        let chain = expect!(value)
            .not()
            .to_be_less_than(30) // "not less than 30" is true for 42
            .and()
            .not()
            .to_be_greater_than(50); // "not greater than 50" is true for 42

        let result = chain.evaluate();
        assert!(result, "NOT chain with AND should evaluate to true when both conditions are true");
    }
}
