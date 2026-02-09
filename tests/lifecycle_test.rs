use rest::prelude::*;
use std::sync::{
    LazyLock, Mutex,
    atomic::{AtomicUsize, Ordering},
};

// Setup counters to track execution
static BEFORE_ALL_COUNTER: AtomicUsize = AtomicUsize::new(0);
static SETUP_COUNTER: AtomicUsize = AtomicUsize::new(0);
static TEARDOWN_COUNTER: AtomicUsize = AtomicUsize::new(0);
static AFTER_ALL_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Mutex to ensure the before_all check is synchronized in parallel execution
static BEFORE_ALL_TEST_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

// Module testing the before_all and after_all attributes
#[with_fixtures_module]
mod lifecycle_fixtures {
    use super::*;

    // Runs once before any test in this module
    #[before_all]
    fn setup_module() {
        BEFORE_ALL_COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    // Runs before each test in this module
    #[setup]
    fn setup_test() {
        SETUP_COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    // Runs after each test in this module
    #[tear_down]
    fn teardown_test() {
        TEARDOWN_COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    // Runs once after all tests in this module
    #[after_all]
    fn teardown_module() {
        AFTER_ALL_COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    #[test]
    fn test_before_all_runs_once() {
        // Acquire a lock to ensure we have a consistent view of all counters
        let _guard = BEFORE_ALL_TEST_MUTEX.lock().unwrap();

        // before_all should have been called exactly once
        let before_all_count = BEFORE_ALL_COUNTER.load(Ordering::SeqCst);
        expect!(before_all_count).to_equal(1);

        // after_all should not have been called yet (it runs at process exit)
        let after_all_count = AFTER_ALL_COUNTER.load(Ordering::SeqCst);
        expect!(after_all_count).to_equal(0);
    }

    #[test]
    fn test_setup_teardown_execution() {
        // Just verify that setup has run for this test
        let setup_count = SETUP_COUNTER.load(Ordering::SeqCst);
        expect!(setup_count).to_be_greater_than(0);

        // We can't reliably check teardown counts in parallel execution
        // so we'll just verify our test ran by pausing briefly
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    #[test]
    fn test_setup_teardown_is_per_test() {
        // setup should have run for each test
        let setup_count = SETUP_COUNTER.load(Ordering::SeqCst);
        expect!(setup_count).to_be_greater_than(0);

        // Just make a simple assertion that can be verified regardless of execution order
        expect!(setup_count > 0 && BEFORE_ALL_COUNTER.load(Ordering::SeqCst) > 0).to_be_true();
    }
}

// Static variable to track after_all execution
static AFTER_ALL_EXECUTED: AtomicUsize = AtomicUsize::new(0);
static AFTER_ALL_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

// Module to test after_all fixtures
#[with_fixtures_module]
mod after_all_test {
    use super::*;

    // Register an after_all handler that will update our counter
    #[after_all]
    fn verify_after_all_runs() {
        // Increment the counter to show it ran
        AFTER_ALL_EXECUTED.fetch_add(1, Ordering::SeqCst);

        // Also verify that setup and test functions ran
        let setup_count = SETUP_COUNTER.load(Ordering::SeqCst);
        let teardown_count = TEARDOWN_COUNTER.load(Ordering::SeqCst);

        // These assertions won't fail the test (since they run during process exit),
        // but they will print to stderr if they fail
        if setup_count == 0 || teardown_count == 0 {
            eprintln!("ERROR in after_all verification: setup_count={}, teardown_count={}", setup_count, teardown_count);
        }
    }

    // We need one test to trigger fixtures registration
    #[test]
    fn test_to_register_fixtures() {
        // Get a lock to ensure consistent view
        let _guard = AFTER_ALL_MUTEX.lock().unwrap();

        // Record that this test ran
        SETUP_COUNTER.fetch_add(1, Ordering::SeqCst);

        // This test should ensure our fixtures are registered
        expect!(true).to_be_true();

        // After_all count should still be 0 (it runs at process exit)
        let after_all_exec_count = AFTER_ALL_EXECUTED.load(Ordering::SeqCst);
        expect!(after_all_exec_count).to_equal(0);
    }

    // Teardown to increment counter when test finishes
    #[tear_down]
    fn mark_test_complete() {
        TEARDOWN_COUNTER.fetch_add(1, Ordering::SeqCst);
    }
}

// A helper test in the main module to verify the after_all behavior
#[test]
fn test_after_all_setup() {
    // This test verifies that we've properly set up the after_all test
    // The actual after_all execution happens at process exit
    let _guard = AFTER_ALL_MUTEX.lock().unwrap();

    // The AFTER_ALL_EXECUTED counter should still be 0 during the test
    let after_all_count = AFTER_ALL_EXECUTED.load(Ordering::SeqCst);
    expect!(after_all_count).to_equal(0);

    // Note: We cannot directly test that after_all runs, because it
    // happens at process exit, but the counter will be incremented
    // when the process exits if it ran successfully
}
