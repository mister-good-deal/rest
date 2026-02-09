use crate::backend::{Assertion, TestSessionResult};
use crate::config::Config;
use crate::events::{AssertionEvent, EventEmitter, on_failure, on_success};
use crate::frontend::ConsoleRenderer;
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::{LazyLock, RwLock};

pub(crate) static GLOBAL_CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| RwLock::new(Config::new()));

thread_local! {
    static TEST_SESSION: RefCell<TestSessionResult> = RefCell::new(TestSessionResult::default());
    // Track already reported messages to avoid duplicates
    static REPORTED_MESSAGES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
    // Flag to enable/disable deduplication
    static DEDUPLICATE_ENABLED: RefCell<bool> = const { RefCell::new(true) };
    // Flag to enable silent mode for intermediate steps in a chain
    static SILENT_MODE: RefCell<bool> = const { RefCell::new(false) };
}

pub struct Reporter;

impl Reporter {
    /// Initialize the reporter with event handlers
    pub fn init() {
        // Register success event handler
        on_success(|result| {
            Self::handle_success_event(result);
        });

        // Register failure event handler
        on_failure(|result| {
            Self::handle_failure_event(result);
        });
    }

    /// Handle success events
    fn handle_success_event(result: Assertion<()>) {
        TEST_SESSION.with(|session| {
            let mut session = session.borrow_mut();
            session.passed_count += 1;
        });

        // Check if silent mode is enabled
        let silent = SILENT_MODE.with(|silent| *silent.borrow());
        if silent {
            return;
        }

        // Check if we should deduplicate
        let should_report = DEDUPLICATE_ENABLED.with(|enabled| {
            if !*enabled.borrow() {
                // Deduplication disabled, always report
                return true;
            }

            // Only report each unique success message once
            REPORTED_MESSAGES.with(|msgs| {
                let key = format!("{:?}", result);
                let mut messages = msgs.borrow_mut();
                if !messages.contains(&key) {
                    messages.insert(key);
                    true
                } else {
                    false
                }
            })
        });

        if should_report {
            let config = GLOBAL_CONFIG.read().unwrap();
            let renderer = ConsoleRenderer::new(Config {
                use_colors: config.use_colors,
                use_unicode_symbols: config.use_unicode_symbols,
                show_success_details: config.show_success_details,
                enhanced_output: config.enhanced_output,
            });
            renderer.print_success(&result);
        }
    }

    /// Handle failure events
    fn handle_failure_event(result: Assertion<()>) {
        TEST_SESSION.with(|session| {
            let mut session = session.borrow_mut();
            session.failed_count += 1;
            session.failures.push(result.clone());
        });

        // Check if silent mode is enabled
        let silent = SILENT_MODE.with(|silent| *silent.borrow());
        if silent {
            return;
        }

        // Check if we should deduplicate
        let should_report = DEDUPLICATE_ENABLED.with(|enabled| {
            if !*enabled.borrow() {
                // Deduplication disabled, always report
                return true;
            }

            // Only report each unique failure message once
            let key = format!("{:?}", result);
            REPORTED_MESSAGES.with(|msgs| {
                let mut messages = msgs.borrow_mut();
                if !messages.contains(&key) {
                    messages.insert(key);
                    true
                } else {
                    false
                }
            })
        });

        if should_report {
            let config = GLOBAL_CONFIG.read().unwrap();
            let renderer = ConsoleRenderer::new(Config {
                use_colors: config.use_colors,
                use_unicode_symbols: config.use_unicode_symbols,
                show_success_details: config.show_success_details,
                enhanced_output: config.enhanced_output,
            });
            renderer.print_failure(&result);
        }
    }

    /// Clear the message cache to allow duplicated messages in different test scopes
    pub fn reset_message_cache() {
        REPORTED_MESSAGES.with(|msgs| {
            msgs.borrow_mut().clear();
        });
    }

    /// Enable deduplication of messages
    pub fn enable_deduplication() {
        DEDUPLICATE_ENABLED.with(|enabled| {
            *enabled.borrow_mut() = true;
        });
    }

    /// Disable deduplication of messages (for tests)
    pub fn disable_deduplication() {
        DEDUPLICATE_ENABLED.with(|enabled| {
            *enabled.borrow_mut() = false;
        });
    }

    /// Enable silent mode to suppress intermediate output in chains
    pub fn enable_silent_mode() {
        SILENT_MODE.with(|silent| {
            *silent.borrow_mut() = true;
        });
    }

    /// Disable silent mode to show all output
    pub fn disable_silent_mode() {
        SILENT_MODE.with(|silent| {
            *silent.borrow_mut() = false;
        });
    }

    pub fn summarize() {
        TEST_SESSION.with(|session| {
            let session = session.borrow();
            let config = GLOBAL_CONFIG.read().unwrap();
            let renderer = ConsoleRenderer::new(Config {
                use_colors: config.use_colors,
                use_unicode_symbols: config.use_unicode_symbols,
                show_success_details: config.show_success_details,
                enhanced_output: config.enhanced_output,
            });
            renderer.print_session_summary(&session);
        });

        // Emit session completed event
        EventEmitter::emit(AssertionEvent::SessionCompleted);

        // Clear reported messages
        Self::reset_message_cache();

        // Reset deduplication to default (enabled)
        Self::enable_deduplication();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::assertions::AssertionStep;
    use crate::backend::assertions::sentence::AssertionSentence;

    // Helper function to create a test assertion that won't evaluate on drop
    fn create_test_assertion(passed: bool) -> Assertion<()> {
        // Create a base assertion
        let mut assertion = Assertion::new((), "test_value");

        // Add a step with the appropriate pass/fail status
        assertion.steps.push(AssertionStep {
            sentence: AssertionSentence::new("be", if passed { "correct" } else { "incorrect" }),
            passed,
            logical_op: None,
        });

        // Set it as non-final to prevent Drop evaluation
        assertion.is_final = false;

        assertion
    }

    // Helper function to just record a failure in the session without actually
    // invoking the full reporter (which would panic on failure)
    fn record_failure(assertion: Assertion<()>) {
        TEST_SESSION.with(|session| {
            let mut session = session.borrow_mut();
            session.failed_count += 1;
            session.failures.push(assertion);
        });
    }

    #[test]
    fn test_reporter_deduplication_flags() {
        // Test enabling and disabling deduplication
        Reporter::enable_deduplication();
        DEDUPLICATE_ENABLED.with(|enabled| {
            assert_eq!(*enabled.borrow(), true);
        });

        Reporter::disable_deduplication();
        DEDUPLICATE_ENABLED.with(|enabled| {
            assert_eq!(*enabled.borrow(), false);
        });

        // Reset to default state
        Reporter::enable_deduplication();
    }

    #[test]
    fn test_reporter_silent_mode() {
        // Test enabling and disabling silent mode
        Reporter::enable_silent_mode();
        SILENT_MODE.with(|silent| {
            assert_eq!(*silent.borrow(), true);
        });

        Reporter::disable_silent_mode();
        SILENT_MODE.with(|silent| {
            assert_eq!(*silent.borrow(), false);
        });
    }

    #[test]
    fn test_reporter_message_cache() {
        // Add a message to the cache
        REPORTED_MESSAGES.with(|msgs| {
            msgs.borrow_mut().insert("test_message".to_string());
        });

        // Verify it's in the cache
        REPORTED_MESSAGES.with(|msgs| {
            assert!(msgs.borrow().contains("test_message"));
        });

        // Reset the cache
        Reporter::reset_message_cache();

        // Verify it's been cleared
        REPORTED_MESSAGES.with(|msgs| {
            assert!(!msgs.borrow().contains("test_message"));
        });
    }

    #[test]
    fn test_handle_success_event() {
        // Start with a clean session
        TEST_SESSION.with(|session| {
            *session.borrow_mut() = TestSessionResult::default();
        });

        // Disable deduplication for this test
        Reporter::disable_deduplication();

        // Create and handle a success event
        let assertion = create_test_assertion(true);
        Reporter::handle_success_event(assertion);

        // Verify the pass count was incremented
        TEST_SESSION.with(|session| {
            let session = session.borrow();
            assert_eq!(session.passed_count, 1);
            assert_eq!(session.failed_count, 0);
            assert_eq!(session.failures.len(), 0);
        });

        // Reset to default state
        Reporter::enable_deduplication();
        Reporter::reset_message_cache();
    }

    #[test]
    fn test_session_tracking() {
        // Start with a clean session
        TEST_SESSION.with(|session| {
            *session.borrow_mut() = TestSessionResult::default();
        });

        // Create a test assertion for failure
        let assertion = create_test_assertion(false);

        // Use our helper to directly record a failure without going through the reporter
        record_failure(assertion.clone());

        // Verify the failure count was incremented and the failure was recorded
        TEST_SESSION.with(|session| {
            let session = session.borrow();
            assert_eq!(session.passed_count, 0);
            assert_eq!(session.failed_count, 1);
            assert_eq!(session.failures.len(), 1);

            // Check that the failure matches what we sent
            if !session.failures.is_empty() {
                let first_failure = &session.failures[0];
                assert_eq!(first_failure.expr_str, assertion.expr_str);
                assert_eq!(first_failure.steps.len(), assertion.steps.len());
                assert_eq!(first_failure.steps[0].passed, assertion.steps[0].passed);
            }
        });

        // Clean up
        TEST_SESSION.with(|session| {
            *session.borrow_mut() = TestSessionResult::default();
        });
    }

    #[test]
    fn test_silent_mode() {
        // Enable silent mode
        Reporter::enable_silent_mode();

        // Verify silent mode is enabled
        SILENT_MODE.with(|silent| {
            assert_eq!(*silent.borrow(), true);
        });

        // Test that success events still increment the counter in silent mode
        // Start with a clean session
        TEST_SESSION.with(|session| {
            *session.borrow_mut() = TestSessionResult::default();
        });

        // Handle a success event in silent mode
        Reporter::handle_success_event(create_test_assertion(true));

        // Verify the pass count was incremented
        TEST_SESSION.with(|session| {
            let session = session.borrow();
            assert_eq!(session.passed_count, 1);
        });

        // Disable silent mode
        Reporter::disable_silent_mode();

        // Verify silent mode is disabled
        SILENT_MODE.with(|silent| {
            assert_eq!(*silent.borrow(), false);
        });

        // Clean up
        TEST_SESSION.with(|session| {
            *session.borrow_mut() = TestSessionResult::default();
        });
    }

    #[test]
    fn test_deduplication() {
        // Enable deduplication
        Reporter::enable_deduplication();
        Reporter::reset_message_cache();

        // Start with a clean session
        TEST_SESSION.with(|session| {
            *session.borrow_mut() = TestSessionResult::default();
        });

        // Create an assertion and send it twice
        let assertion = create_test_assertion(true);

        // Handle the same success event twice
        Reporter::handle_success_event(assertion.clone());
        Reporter::handle_success_event(assertion);

        // We should only count it once due to deduplication
        REPORTED_MESSAGES.with(|msgs| {
            assert_eq!(msgs.borrow().len(), 1);
        });

        // Verify it was still counted twice in the session
        TEST_SESSION.with(|session| {
            let session = session.borrow();
            assert_eq!(session.passed_count, 2);
        });

        // Clean up
        Reporter::reset_message_cache();
    }
}
