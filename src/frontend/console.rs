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
                if let Some(ref actual) = step.sentence.actual_value {
                    format!("{} (got {})", base, actual)
                } else {
                    base
                }
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
