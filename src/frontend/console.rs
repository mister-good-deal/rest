use crate::backend::LogicalOp;
use crate::backend::{Assertion, TestSessionResult};
use crate::config::Config;
use colored::*;

/// Handles rendering of test results to the console
pub struct ConsoleRenderer {
    config: Config,
}

impl ConsoleRenderer {
    /// Create a new renderer with the provided configuration
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Render a successful assertion result
    pub fn render_success(&self, result: &Assertion<()>) -> String {
        let message = self.build_assertion_message(result);

        if self.config.show_success_details {
            let prefix = if self.config.use_unicode_symbols { "✓ " } else { "+ " };
            if self.config.use_colors {
                return format!("{}{}", prefix.green(), message.green());
            } else {
                return format!("{}{}", prefix, message);
            }
        } else {
            return String::new(); // Empty string when not showing success details
        }
    }

    /// Render a failed assertion result
    pub fn render_failure(&self, result: &Assertion<()>) -> (String, String) {
        let message = self.build_assertion_message(result);
        let details = self.build_failure_details(result);

        let prefix = if self.config.use_unicode_symbols { "✗ " } else { "- " };
        let header = if self.config.use_colors { format!("{}{}", prefix, message.red().bold()) } else { format!("{}{}", prefix, message) };

        return (header, details);
    }

    /// Build a failure details string
    fn build_failure_details(&self, result: &Assertion<()>) -> String {
        let mut details = String::new();

        // Add individual step results with proper formatting
        for step in &result.steps {
            let result_symbol = if step.passed { "✓" } else { "✗" };
            // For individual steps, conjugate based on the subject name
            let formatted_sentence = if step.passed {
                step.sentence.format_with_conjugation(result.expr_str)
            } else {
                // On failure, append the actual value for better diagnostics
                let base = step.sentence.format_with_conjugation(result.expr_str);
                if let Some(ref actual) = step.sentence.actual_value { format!("{} (got {})", base, actual) } else { base }
            };

            // Always indent and add pass/fail prefix
            details.push_str(&format!("  {} {}\n", result_symbol, formatted_sentence));
        }

        return details;
    }

    /// Build the main assertion message
    fn build_assertion_message(&self, result: &Assertion<()>) -> String {
        if result.steps.is_empty() {
            return "No assertions made".to_string();
        }

        // Clean expression string (remove reference symbols)
        let clean_expr = result.expr_str.trim_start_matches('&');

        // For single assertions, conjugate based on the subject name
        if result.steps.len() == 1 {
            return format!("{} {}", clean_expr, result.steps[0].sentence.format_with_conjugation(result.expr_str));
        }

        // Start with the first step and conjugate based on the subject
        let mut message = format!("{} {}", clean_expr, result.steps[0].sentence.format_with_conjugation(result.expr_str));

        // Add remaining steps with logical operators
        for i in 1..result.steps.len() {
            let prev = &result.steps[i - 1];
            let curr = &result.steps[i];

            let op_str = match prev.logical_op {
                Some(LogicalOp::And) => " AND ",
                Some(LogicalOp::Or) => " OR ",
                None => " [MISSING OP] ",
            };

            // For all subsequent parts in a chain, use conjugated verbs with grammatical format for consistency
            // This makes phrases like "is greater than X AND is less than Y" instead of "is greater than X AND be less than Y"
            message.push_str(&format!("{}{}", op_str, curr.sentence.format_with_conjugation(result.expr_str)));
        }

        return message;
    }

    /// Render a full test session result
    pub fn render_session_summary(&self, result: &TestSessionResult) -> String {
        let mut output = String::from("\nTest Results:\n");

        let passed_msg = format!("{} passed", result.passed_count);
        let failed_msg = format!("{} failed", result.failed_count);

        if self.config.use_colors {
            output.push_str(&format!(
                "  {} / {}\n",
                if result.passed_count > 0 { passed_msg.green() } else { passed_msg.normal() },
                if result.failed_count > 0 { failed_msg.red().bold() } else { failed_msg.normal() }
            ));
        } else {
            output.push_str(&format!("  {} / {}\n", passed_msg, failed_msg));
        }

        if result.failed_count > 0 {
            output.push_str("\nFailure Details:\n");
            for (i, failure) in result.failures.iter().enumerate() {
                let (header, details) = self.render_failure(failure);
                output.push_str(&format!("  {}. {}\n", i + 1, header));

                // Process each line of the details with indentation
                for line in details.lines() {
                    output.push_str(&format!("     {}\n", line));
                }
            }
        }

        return output;
    }

    /// Format and print a successful test result to the console
    pub fn print_success(&self, result: &Assertion<()>) {
        let message = self.render_success(result);
        if !message.is_empty() {
            println!("{}", message);
        }
    }

    /// Format and print a failed test result to the console
    pub fn print_failure(&self, result: &Assertion<()>) {
        let (header, details) = self.render_failure(result);

        // Print the main error message
        println!("{}", header);

        // Print the details with appropriate colors
        if self.config.use_colors {
            for line in details.lines() {
                if line.contains("✓") {
                    println!("{}", line.green());
                } else if line.contains("✗") {
                    println!("{}", line.red());
                } else {
                    println!("{}", line);
                }
            }
        } else {
            // Print without colors
            println!("{}", details);
        }
    }

    /// Print the complete test session summary
    pub fn print_session_summary(&self, result: &TestSessionResult) {
        println!("{}", self.render_session_summary(result));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::assertions::sentence::AssertionSentence;
    use crate::backend::{Assertion, AssertionStep, LogicalOp, TestSessionResult};

    fn make_config(colors: bool, unicode: bool) -> Config {
        Config::new().use_colors(colors).use_unicode_symbols(unicode).show_success_details(true).enhanced_output(true)
    }

    fn make_passed_assertion(expr: &'static str, verb: &str, object: &str) -> Assertion<()> {
        let mut assertion = Assertion::new((), expr);
        assertion.evaluated = true;
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new(verb, object).with_negation(false),
            passed: true,
            logical_op: None,
        });
        assertion
    }

    fn make_failed_assertion(expr: &'static str, verb: &str, object: &str, actual: Option<&str>) -> Assertion<()> {
        let mut assertion = Assertion::new((), expr);
        assertion.evaluated = true;
        let mut sentence = AssertionSentence::new(verb, object).with_negation(false);
        if let Some(actual_val) = actual {
            sentence = sentence.with_actual(actual_val);
        }
        assertion.steps.push(AssertionStep { sentence, passed: false, logical_op: None });
        assertion
    }

    // ---------- render_success ----------

    #[test]
    fn render_success_with_unicode_and_colors() {
        let renderer = ConsoleRenderer::new(make_config(true, true));
        let assertion = make_passed_assertion("value", "be", "true");
        let output = renderer.render_success(&assertion);

        // Check that the raw (unescaped) output contains the expected prefix and message
        assert!(output.contains("✓"), "should contain unicode checkmark");
        assert!(output.contains("value"), "should contain the expression");
    }

    #[test]
    fn render_success_with_unicode_no_colors() {
        let renderer = ConsoleRenderer::new(make_config(false, true));
        let assertion = make_passed_assertion("value", "be", "true");
        let output = renderer.render_success(&assertion);

        assert!(output.starts_with("✓ "), "should start with unicode checkmark");
        assert!(output.contains("value"), "should contain the expression");
    }

    #[test]
    fn render_success_without_unicode_no_colors() {
        let renderer = ConsoleRenderer::new(make_config(false, false));
        let assertion = make_passed_assertion("value", "be", "true");
        let output = renderer.render_success(&assertion);

        assert!(output.starts_with("+ "), "should start with + prefix");
        assert!(output.contains("value"), "should contain the expression");
    }

    #[test]
    fn render_success_hidden_when_show_success_disabled() {
        let config = Config::new().use_colors(false).use_unicode_symbols(false).show_success_details(false).enhanced_output(true);
        let renderer = ConsoleRenderer::new(config);
        let assertion = make_passed_assertion("value", "be", "true");
        let output = renderer.render_success(&assertion);

        assert!(output.is_empty(), "should return empty string when details disabled");
    }

    #[test]
    fn render_success_no_assertions() {
        let renderer = ConsoleRenderer::new(make_config(false, false));
        let mut assertion = Assertion::new((), "x");
        assertion.evaluated = true;
        let output = renderer.render_success(&assertion);

        assert!(output.contains("No assertions made"), "should note no assertions");
    }

    // ---------- render_failure ----------

    #[test]
    fn render_failure_basic_no_colors() {
        let renderer = ConsoleRenderer::new(make_config(false, false));
        let assertion = make_failed_assertion("count", "be", "greater than 10", Some("5"));
        let (header, details) = renderer.render_failure(&assertion);

        assert!(header.starts_with("- "), "should start with - prefix");
        assert!(header.contains("count"), "header should contain expression");
        assert!(details.contains("✗"), "details should contain failure symbol");
        assert!(details.contains("got 5"), "details should contain actual value");
    }

    #[test]
    fn render_failure_with_unicode_no_colors() {
        let renderer = ConsoleRenderer::new(make_config(false, true));
        let assertion = make_failed_assertion("x", "be", "even", None);
        let (header, _details) = renderer.render_failure(&assertion);

        assert!(header.starts_with("✗ "), "should start with unicode cross");
    }

    #[test]
    fn render_failure_without_unicode_no_colors() {
        let renderer = ConsoleRenderer::new(make_config(false, false));
        let assertion = make_failed_assertion("x", "be", "even", None);
        let (header, _details) = renderer.render_failure(&assertion);

        assert!(header.starts_with("- "), "should start with - prefix");
    }

    #[test]
    fn render_failure_with_negation() {
        let renderer = ConsoleRenderer::new(make_config(false, false));
        let mut assertion = Assertion::new((), "value");
        assertion.evaluated = true;
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "positive").with_negation(true),
            passed: false,
            logical_op: None,
        });
        let (header, details) = renderer.render_failure(&assertion);

        assert!(header.contains("not"), "header should show negation");
        assert!(details.contains("✗"), "details should contain failure symbol");
    }

    #[test]
    fn render_failure_with_colors() {
        let renderer = ConsoleRenderer::new(make_config(true, true));
        let assertion = make_failed_assertion("x", "be", "positive", None);
        let (header, _details) = renderer.render_failure(&assertion);

        // When colors are enabled, ANSI codes are embedded.
        // Just ensure non-empty output with expression content.
        assert!(!header.is_empty(), "header should not be empty with colors");
        assert!(header.contains("x"), "header should contain expression even with color codes");
    }

    // ---------- multi-step assertions ----------

    #[test]
    fn render_success_multi_step_and() {
        let renderer = ConsoleRenderer::new(make_config(false, false));
        let mut assertion = Assertion::new((), "n");
        assertion.evaluated = true;
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "greater than 3").with_negation(false),
            passed: true,
            logical_op: Some(LogicalOp::And),
        });
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "less than 10").with_negation(false),
            passed: true,
            logical_op: None,
        });
        let output = renderer.render_success(&assertion);

        assert!(output.contains("AND"), "should contain AND connector");
        assert!(output.contains("greater than 3"), "should contain first step");
        assert!(output.contains("less than 10"), "should contain second step");
    }

    #[test]
    fn render_success_multi_step_or() {
        let renderer = ConsoleRenderer::new(make_config(false, false));
        let mut assertion = Assertion::new((), "n");
        assertion.evaluated = true;
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "equal to 3").with_negation(false),
            passed: false,
            logical_op: Some(LogicalOp::Or),
        });
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "equal to 5").with_negation(false),
            passed: true,
            logical_op: None,
        });
        let output = renderer.render_success(&assertion);

        assert!(output.contains("OR"), "should contain OR connector");
    }

    #[test]
    fn render_failure_multi_step_details() {
        let renderer = ConsoleRenderer::new(make_config(false, false));
        let mut assertion = Assertion::new((), "val");
        assertion.evaluated = true;
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "greater than 3").with_negation(false),
            passed: true,
            logical_op: Some(LogicalOp::And),
        });
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "less than 2").with_negation(false).with_actual("5"),
            passed: false,
            logical_op: None,
        });
        let (_header, details) = renderer.render_failure(&assertion);

        assert!(details.contains("✓"), "should mark first step as passed");
        assert!(details.contains("✗"), "should mark second step as failed");
        assert!(details.contains("got 5"), "should contain actual value for failed step");
    }

    #[test]
    fn render_failure_missing_logical_op() {
        let renderer = ConsoleRenderer::new(make_config(false, false));
        let mut assertion = Assertion::new((), "x");
        assertion.evaluated = true;
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "positive").with_negation(false),
            passed: true,
            logical_op: None, // Missing op between steps
        });
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "even").with_negation(false),
            passed: false,
            logical_op: None,
        });
        let (header, _details) = renderer.render_failure(&assertion);

        assert!(header.contains("[MISSING OP]"), "should show missing op marker");
    }

    // ---------- render_session_summary ----------

    #[test]
    fn render_session_summary_all_passed() {
        let renderer = ConsoleRenderer::new(make_config(false, false));
        let result = TestSessionResult { passed_count: 5, failed_count: 0, failures: vec![] };
        let output = renderer.render_session_summary(&result);

        assert!(output.contains("5 passed"), "should show passed count");
        assert!(output.contains("0 failed"), "should show failed count");
        assert!(!output.contains("Failure Details"), "should not show failure section");
    }

    #[test]
    fn render_session_summary_with_failures() {
        let renderer = ConsoleRenderer::new(make_config(false, false));
        let failure = make_failed_assertion("x", "be", "positive", Some("-1"));
        let result = TestSessionResult { passed_count: 3, failed_count: 1, failures: vec![failure] };
        let output = renderer.render_session_summary(&result);

        assert!(output.contains("3 passed"), "should show passed count");
        assert!(output.contains("1 failed"), "should show failed count");
        assert!(output.contains("Failure Details"), "should show failure section");
        assert!(output.contains("1."), "should number failures");
    }

    #[test]
    fn render_session_summary_with_colors() {
        let renderer = ConsoleRenderer::new(make_config(true, true));
        let result = TestSessionResult { passed_count: 2, failed_count: 1, failures: vec![make_failed_assertion("x", "be", "true", None)] };
        let output = renderer.render_session_summary(&result);

        // Just check that it renders something sensible
        assert!(output.contains("Test Results"), "should contain header");
        assert!(output.contains("passed"), "should mention passed");
        assert!(output.contains("failed"), "should mention failed");
        assert!(output.contains("Failure Details"), "should contain failure section");
    }

    #[test]
    fn render_session_summary_zero_passed_and_failed() {
        let renderer = ConsoleRenderer::new(make_config(false, false));
        let result = TestSessionResult { passed_count: 0, failed_count: 0, failures: vec![] };
        let output = renderer.render_session_summary(&result);

        assert!(output.contains("0 passed"), "should show 0 passed");
        assert!(output.contains("0 failed"), "should show 0 failed");
    }

    // ---------- edge cases ----------

    #[test]
    fn render_success_with_reference_expression() {
        let renderer = ConsoleRenderer::new(make_config(false, false));
        let mut assertion = Assertion::new((), "&my_var");
        assertion.evaluated = true;
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "true").with_negation(false),
            passed: true,
            logical_op: None,
        });
        let output = renderer.render_success(&assertion);

        // The & should be stripped from the expression
        assert!(output.contains("my_var"), "should strip reference symbol from expression");
        assert!(!output.contains("&my_var"), "should not contain raw reference expression");
    }

    #[test]
    fn render_failure_multiple_failures_in_summary() {
        let renderer = ConsoleRenderer::new(make_config(false, false));
        let f1 = make_failed_assertion("a", "be", "positive", Some("-1"));
        let f2 = make_failed_assertion("b", "be", "even", Some("3"));
        let result = TestSessionResult { passed_count: 0, failed_count: 2, failures: vec![f1, f2] };
        let output = renderer.render_session_summary(&result);

        assert!(output.contains("1."), "should number first failure");
        assert!(output.contains("2."), "should number second failure");
        assert!(output.contains("2 failed"), "should show total failed");
    }

    #[test]
    fn render_success_three_step_chain() {
        let renderer = ConsoleRenderer::new(make_config(false, false));
        let mut assertion = Assertion::new((), "n");
        assertion.evaluated = true;
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "greater than 1").with_negation(false),
            passed: true,
            logical_op: Some(LogicalOp::And),
        });
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "less than 100").with_negation(false),
            passed: true,
            logical_op: Some(LogicalOp::And),
        });
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", "odd").with_negation(false),
            passed: true,
            logical_op: None,
        });
        let output = renderer.render_success(&assertion);

        // Should contain two AND connectors
        let and_count = output.matches("AND").count();
        assert_eq!(and_count, 2, "should have two AND connectors");
    }
}
