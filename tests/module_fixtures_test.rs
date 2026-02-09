use rest::prelude::*;
use std::cell::RefCell;
use std::sync::{
    LazyLock, Mutex,
    atomic::{AtomicUsize, Ordering},
};

// Counters for tracking setup and teardown calls
static SETUP_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Mutex for synchronizing access to the test value
static TEST_VALUE_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
static INNER_TEST_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
static OUTER_TEST_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

// Test state shared between tests
thread_local! {
    static TEST_VALUE: RefCell<u32> = RefCell::new(0);
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

// Main test module using module-level fixtures
#[with_fixtures_module]
mod module_fixtures {
    use super::*;

    // Setup function using the attribute style
    #[setup]
    fn setup_function() {
        SETUP_COUNTER.fetch_add(1, Ordering::SeqCst);
        TEST_VALUE.with(|v| {
            *v.borrow_mut() = 0;
        });
    }

    // Teardown function using the attribute style
    #[tear_down]
    fn teardown_function() {
        expect!(get_test_value()).to_equal(42);
    }

    // This test should have fixtures applied automatically
    #[test]
    fn test_fixtures_are_applied_automatically() {
        // Acquire the lock to ensure consistent state
        let _guard = TEST_VALUE_MUTEX.lock().unwrap();

        // Setup function should have been called
        let setup_count = SETUP_COUNTER.load(Ordering::SeqCst);
        expect!(setup_count).to_be_greater_than(0);

        // Test value should be reset by setup
        expect!(get_test_value()).to_equal(0);

        // Modify the value
        set_test_value(42);
        expect!(get_test_value()).to_equal(42);

        // Small delay for consistency
        std::thread::sleep(std::time::Duration::from_millis(5));
    }

    // This test should also have fixtures applied automatically
    #[test]
    fn test_fixtures_run_for_each_test() {
        // Acquire the lock to ensure consistent state
        let _guard = TEST_VALUE_MUTEX.lock().unwrap();

        // Value should be reset back to 0 for this test
        expect!(get_test_value()).to_equal(0);

        // Setup should have run multiple times by now if tests are run sequentially,
        // but might not be true in parallel execution
        let setup_count = SETUP_COUNTER.load(Ordering::SeqCst);
        expect!(setup_count).to_be_greater_than(0);

        // Modify the value
        set_test_value(42);
        expect!(get_test_value()).to_equal(42);

        // Small delay for consistency
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

// A nested module test case
mod nested_module_test {
    use super::*;

    // The outer module has a different setup/teardown
    #[setup]
    fn outer_setup() {
        SETUP_COUNTER.fetch_add(10, Ordering::SeqCst); // Add 10 to differentiate
        TEST_VALUE.with(|v| {
            *v.borrow_mut() = 100;
        });
    }

    #[tear_down]
    fn outer_teardown() {
        expect!(get_test_value()).to_equal(150);
    }

    // This nested module gets its own fixtures
    #[with_fixtures_module]
    mod inner_module {
        use super::*;

        // Override the setup defined in the outer module
        #[setup]
        fn inner_setup() {
            SETUP_COUNTER.fetch_add(1, Ordering::SeqCst);
            // Reset to a specific value to verify this setup runs
            TEST_VALUE.with(|v| {
                *v.borrow_mut() = 200;
            });
        }

        #[tear_down]
        fn inner_teardown() {
            // This teardown should check the value set by inner_setup
            expect!(get_test_value()).to_equal(250);
        }

        // This test should use the inner module's fixtures
        #[test]
        fn test_inner_fixtures_are_applied() {
            // Acquire inner module lock
            let _guard = INNER_TEST_MUTEX.lock().unwrap();

            // Test should have the value from the inner setup, not outer setup
            expect!(get_test_value()).to_equal(200);
            // Modify the value
            set_test_value(250);
            expect!(get_test_value()).to_equal(250);

            // Small delay for stability
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }

    // Test that uses the outer fixtures but needs explicit annotation
    #[test]
    #[with_fixtures]
    fn test_outer_fixtures_explicit() {
        // Acquire outer module lock
        let _guard = OUTER_TEST_MUTEX.lock().unwrap();

        // This should use the outer setup, which sets to 100
        expect!(get_test_value()).to_equal(100);
        // Modify the value
        set_test_value(150);
        expect!(get_test_value()).to_equal(150);

        // Small delay for stability
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}
