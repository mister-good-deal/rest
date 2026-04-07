use crate::backend::Assertion;
use std::cell::RefCell;

/// Event types that can be emitted within the testing system
#[derive(Debug, Clone)]
pub enum AssertionEvent {
    /// A successful assertion
    Success(Assertion<()>),
    /// A failed assertion
    Failure(Assertion<()>),
    /// Test session completed
    SessionCompleted,
}

// Thread-local registry of success handlers
// Define type aliases to reduce complexity
type AssertionHandler = Box<dyn Fn(Assertion<()>)>;

thread_local! {
    static SUCCESS_HANDLERS: RefCell<Vec<AssertionHandler>> = RefCell::new(Vec::new());
    static FAILURE_HANDLERS: RefCell<Vec<AssertionHandler>> = RefCell::new(Vec::new());
    static SESSION_COMPLETED_HANDLERS: RefCell<Vec<Box<dyn Fn()>>> = RefCell::new(Vec::new());
    static INITIALIZED: RefCell<bool> = const { RefCell::new(false) };
}

/// EventEmitter is responsible for sending events and managing event handlers
pub struct EventEmitter;

impl EventEmitter {
    /// Initialize the event system
    pub fn init() {
        INITIALIZED.with(|initialized| {
            let mut initialized = initialized.borrow_mut();
            if !*initialized {
                *initialized = true;
            }
        });
    }

    /// Emit an event to all registered handlers
    ///
    /// Handlers are temporarily taken out of the registry before being called,
    /// so the RefCell borrow is not held during handler execution. This allows
    /// handlers to safely trigger code that registers new handlers (e.g.
    /// Assertion::drop → initialize() → Reporter::init() → on_success()).
    pub fn emit(event: AssertionEvent) {
        match event {
            AssertionEvent::Success(mut assertion) => {
                assertion.evaluated = true;
                SUCCESS_HANDLERS.with(|cell| {
                    let taken = cell.replace(Vec::new());
                    for handler in taken.iter() {
                        handler(assertion.clone());
                    }
                    let mut new_during_emit = cell.replace(taken);
                    cell.borrow_mut().append(&mut new_during_emit);
                });
            }
            AssertionEvent::Failure(mut assertion) => {
                assertion.evaluated = true;
                FAILURE_HANDLERS.with(|cell| {
                    let taken = cell.replace(Vec::new());
                    for handler in taken.iter() {
                        handler(assertion.clone());
                    }
                    let mut new_during_emit = cell.replace(taken);
                    cell.borrow_mut().append(&mut new_during_emit);
                });
            }
            AssertionEvent::SessionCompleted => {
                SESSION_COMPLETED_HANDLERS.with(|cell| {
                    let taken = cell.replace(Vec::new());
                    for handler in taken.iter() {
                        handler();
                    }
                    let mut new_during_emit = cell.replace(taken);
                    cell.borrow_mut().append(&mut new_during_emit);
                });
            }
        }
    }
}

/// Register a handler for success events
pub fn on_success<F>(handler: F)
where
    F: Fn(Assertion<()>) + 'static,
{
    SUCCESS_HANDLERS.with(|handlers| {
        handlers.borrow_mut().push(Box::new(handler));
    });
}

/// Register a handler for failure events
pub fn on_failure<F>(handler: F)
where
    F: Fn(Assertion<()>) + 'static,
{
    FAILURE_HANDLERS.with(|handlers| {
        handlers.borrow_mut().push(Box::new(handler));
    });
}

/// Register a handler for session completion events
pub fn on_session_completed<F>(handler: F)
where
    F: Fn() + 'static,
{
    SESSION_COMPLETED_HANDLERS.with(|handlers| {
        handlers.borrow_mut().push(Box::new(handler));
    });
}

/// Clear all handler registries. Used in tests to ensure each test starts
/// with a clean slate, preventing handler accumulation across tests that share
/// the same thread.
#[cfg(test)]
pub fn reset_handlers() {
    SUCCESS_HANDLERS.with(|h| h.borrow_mut().clear());
    FAILURE_HANDLERS.with(|h| h.borrow_mut().clear());
    SESSION_COMPLETED_HANDLERS.with(|h| h.borrow_mut().clear());
}

// This is an internal function, deprecated in favor of using Config.apply()
// but kept for compatibility with example and test code
#[doc(hidden)]
pub fn initialize_event_system() {
    EventEmitter::init();

    // Register default handlers from the Reporter
    crate::Reporter::init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::assertions::AssertionStep;
    use crate::backend::assertions::sentence::AssertionSentence;
    use std::cell::RefCell;
    use std::rc::Rc;

    // Create a test assertion
    fn create_test_assertion() -> Assertion<()> {
        let mut assertion = Assertion::new((), "test_value");
        assertion.steps.push(AssertionStep { sentence: AssertionSentence::new("be", "test assertion"), passed: true, logical_op: None });
        assertion
    }

    #[test]
    fn test_event_emitter_init() {
        reset_handlers();
        // This will initialize the event system
        EventEmitter::init();

        // Check that the initialization worked
        INITIALIZED.with(|initialized| {
            assert_eq!(*initialized.borrow(), true);
        });
    }

    #[test]
    fn test_on_success_handler() {
        reset_handlers();
        // Create a flag to check if the handler was called
        let called = Rc::new(RefCell::new(false));
        let called_clone = called.clone();

        // Register a success handler
        on_success(move |_| {
            *called.borrow_mut() = true;
        });

        // Emit a success event
        let assertion = create_test_assertion();
        EventEmitter::emit(AssertionEvent::Success(assertion));

        // Check that the handler was called
        assert_eq!(*called_clone.borrow(), true);
    }

    #[test]
    fn test_on_failure_handler() {
        reset_handlers();
        // Create a flag to check if the handler was called
        let called = Rc::new(RefCell::new(false));
        let called_clone = called.clone();

        // Register a failure handler
        on_failure(move |_| {
            *called.borrow_mut() = true;
        });

        // Emit a failure event
        let assertion = create_test_assertion();
        EventEmitter::emit(AssertionEvent::Failure(assertion));

        // Check that the handler was called
        assert_eq!(*called_clone.borrow(), true);
    }

    #[test]
    fn test_on_session_completed_handler() {
        reset_handlers();
        // Create a flag to check if the handler was called
        let called = Rc::new(RefCell::new(false));
        let called_clone = called.clone();

        // Register a session completed handler
        on_session_completed(move || {
            *called.borrow_mut() = true;
        });

        // Emit a session completed event
        EventEmitter::emit(AssertionEvent::SessionCompleted);

        // Check that the handler was called
        assert_eq!(*called_clone.borrow(), true);
    }

    #[test]
    fn test_multiple_handlers() {
        reset_handlers();
        // Create counters for each handler type
        let success_count = Rc::new(RefCell::new(0));
        let success_count_clone = success_count.clone();

        let failure_count = Rc::new(RefCell::new(0));
        let failure_count_clone = failure_count.clone();

        let session_count = Rc::new(RefCell::new(0));
        let session_count_clone = session_count.clone();

        // Register multiple handlers of each type
        for _ in 0..3 {
            let count = success_count.clone();
            on_success(move |_| {
                *count.borrow_mut() += 1;
            });

            let count = failure_count.clone();
            on_failure(move |_| {
                *count.borrow_mut() += 1;
            });

            let count = session_count.clone();
            on_session_completed(move || {
                *count.borrow_mut() += 1;
            });
        }

        // Emit events
        let assertion = create_test_assertion();
        EventEmitter::emit(AssertionEvent::Success(assertion.clone()));
        EventEmitter::emit(AssertionEvent::Failure(assertion));
        EventEmitter::emit(AssertionEvent::SessionCompleted);

        // Check that all handlers were called
        assert_eq!(*success_count_clone.borrow(), 3);
        assert_eq!(*failure_count_clone.borrow(), 3);
        assert_eq!(*session_count_clone.borrow(), 3);
    }

    #[test]
    fn test_assertion_event_debug() {
        reset_handlers();
        // Test that the Debug implementation works
        let assertion = create_test_assertion();
        let success_event = AssertionEvent::Success(assertion.clone());
        let failure_event = AssertionEvent::Failure(assertion);
        let session_event = AssertionEvent::SessionCompleted;

        // Make sure these don't panic and assert that they produce non-empty strings
        assert!(!format!("{:?}", success_event).is_empty());
        assert!(!format!("{:?}", failure_event).is_empty());
        assert!(!format!("{:?}", session_event).is_empty());
    }
}
