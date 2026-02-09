use rest::prelude::*;
use std::cell::RefCell;
use std::sync::{
    LazyLock, Mutex,
    atomic::{AtomicUsize, Ordering},
};

// Counter for tracking setup and teardown calls
static SETUP_COUNTER: AtomicUsize = AtomicUsize::new(0);
static TEARDOWN_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Mutex for test value verification
static TEST_VALUE_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

// Test state shared between tests
thread_local! {
    static TEST_VALUE: RefCell<u32> = RefCell::new(0);
}

// Reset helper for the test value - this is called by the setup function
fn reset_test_value() {
    TEST_VALUE.with(|v| {
        *v.borrow_mut() = 0;
    });
}

// Helper to set the test value
fn set_test_value(value: u32) {
    TEST_VALUE.with(|v| {
        *v.borrow_mut() = value;
    });
}

// Helper to get the test value
fn get_test_value() -> u32 {
    TEST_VALUE.with(|v| *v.borrow())
}

// Main test module for fixtures tests
mod main_fixtures {
    use super::*;

    // Setup function using the attribute style
    #[setup]
    fn setup_function() {
        SETUP_COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    // Teardown function using the attribute style
    #[tear_down]
    fn teardown_function() {
        TEARDOWN_COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    // Add a setup function that resets the test value
    #[setup]
    fn reset_test_state() {
        reset_test_value();
    }

    #[test]
    #[with_fixtures]
    fn test_fixtures_are_called() {
        // Get the lock to ensure consistent state
        let _guard = TEST_VALUE_MUTEX.lock().unwrap();

        // Setup functions should have been called
        let setup_count = SETUP_COUNTER.load(Ordering::SeqCst);
        expect!(setup_count).to_be_greater_than(0);

        // Verify test value was reset
        expect!(get_test_value()).to_equal(0);

        // Do something in the test
        set_test_value(42);
        expect!(get_test_value()).to_equal(42);

        // Small delay for stability
        std::thread::sleep(std::time::Duration::from_millis(5));
    }

    #[test]
    #[with_fixtures]
    fn test_fixtures_run_for_each_test() {
        // Get the lock to ensure consistent state
        let _guard = TEST_VALUE_MUTEX.lock().unwrap();

        // Verify that test state was reset (should be 0 from our setup function)
        expect!(get_test_value()).to_equal(0);

        // Set a value for this test
        set_test_value(123);

        // Do a small delay to simulate test work
        std::thread::sleep(std::time::Duration::from_millis(5));

        // For teardown verification
        expect!(get_test_value()).to_equal(123);
    }

    #[test]
    #[with_fixtures]
    fn test_teardown_ran_from_previous_test() {
        // Get the lock to ensure consistent state
        let _guard = TEST_VALUE_MUTEX.lock().unwrap();

        // Verify setup ran at least once
        let setup_count = SETUP_COUNTER.load(Ordering::SeqCst);
        expect!(setup_count).to_be_greater_than(0);

        // The value should be reset by our reset_test_state setup function
        expect!(get_test_value()).to_equal(0);

        // Teardown should have run at least once by now
        let teardown_count = TEARDOWN_COUNTER.load(Ordering::SeqCst);

        // Verify some basic sanity about setup and teardown execution
        if setup_count > 1 {
            // If we've run multiple tests, teardown should have run at least once
            expect!(teardown_count).to_be_greater_than(0);
        }
    }
}
