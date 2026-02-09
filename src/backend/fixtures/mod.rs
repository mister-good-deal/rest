//! Module for test fixtures support with setup and teardown capabilities
//!
//! This module provides the runtime functionality for test fixtures using attributes.
//! It works with procedural macros to provide a clean API for setting up and tearing
//! down test environments.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::panic::{self, AssertUnwindSafe};
use std::sync::{LazyLock, Mutex};

/// Simple fixture registration system that uses a global hashmap instead of inventory
pub type FixtureFunc = Box<dyn Fn() + Send + Sync + 'static>;

static SETUP_FIXTURES: LazyLock<Mutex<HashMap<&'static str, Vec<FixtureFunc>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

static TEARDOWN_FIXTURES: LazyLock<Mutex<HashMap<&'static str, Vec<FixtureFunc>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

static BEFORE_ALL_FIXTURES: LazyLock<Mutex<HashMap<&'static str, Vec<FixtureFunc>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

static AFTER_ALL_FIXTURES: LazyLock<Mutex<HashMap<&'static str, Vec<FixtureFunc>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

static EXECUTED_MODULES: LazyLock<Mutex<HashSet<&'static str>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

/// Register a setup function for a module
///
/// This is automatically called by the `#[setup]` attribute macro.
pub fn register_setup(module_path: &'static str, func: FixtureFunc) {
    let mut fixtures = SETUP_FIXTURES.lock().unwrap();
    fixtures.entry(module_path).or_default().push(func);
}

/// Register a teardown function for a module
///
/// This is automatically called by the `#[tear_down]` attribute macro.
pub fn register_teardown(module_path: &'static str, func: FixtureFunc) {
    let mut fixtures = TEARDOWN_FIXTURES.lock().unwrap();
    fixtures.entry(module_path).or_default().push(func);
}

/// Register a before_all function for a module
///
/// This is automatically called by the `#[before_all]` attribute macro.
/// These functions run once before any test in the module.
pub fn register_before_all(module_path: &'static str, func: FixtureFunc) {
    let mut fixtures = BEFORE_ALL_FIXTURES.lock().unwrap();
    fixtures.entry(module_path).or_default().push(func);
}

/// Register an after_all function for a module
///
/// This is automatically called by the `#[after_all]` attribute macro.
/// These functions run once after all tests in the module.
/// Note: In standalone test execution, this is guaranteed to run.
/// But in parallel test execution, it depends on the test runner.
pub fn register_after_all(module_path: &'static str, func: FixtureFunc) {
    let mut fixtures = AFTER_ALL_FIXTURES.lock().unwrap();
    fixtures.entry(module_path).or_default().push(func);
}

thread_local! {
    /// Indicator of whether we're currently in a fixture-wrapped test
    static IN_FIXTURE_TEST: RefCell<bool> = const { RefCell::new(false) };
}

/// Run a test function with appropriate setup and teardown
///
/// This is automatically called by the `#[with_fixtures]` attribute macro.
pub fn run_test_with_fixtures<F>(module_path: &'static str, test_fn: AssertUnwindSafe<F>)
where
    F: FnOnce(),
{
    // Set the fixture test flag
    IN_FIXTURE_TEST.with(|flag| {
        *flag.borrow_mut() = true;
    });

    // Check if before_all fixtures have been run for this module
    // and run them if they haven't
    run_before_all_if_needed(module_path);

    // Run setup functions for this module if any exist
    if let Ok(fixtures) = SETUP_FIXTURES.lock()
        && let Some(setup_funcs) = fixtures.get(module_path)
    {
        for setup_fn in setup_funcs {
            setup_fn();
        }
    }

    // Run the test function, capturing any panics
    let result = panic::catch_unwind(test_fn);

    // Always run teardown, even if the test panics
    if let Ok(fixtures) = TEARDOWN_FIXTURES.lock()
        && let Some(teardown_funcs) = fixtures.get(module_path)
    {
        for teardown_fn in teardown_funcs {
            teardown_fn();
        }
    }

    // Reset the fixture test flag
    IN_FIXTURE_TEST.with(|flag| {
        *flag.borrow_mut() = false;
    });

    // Register after_all fixtures to be run at process exit
    // We can't run them now because we don't know if this is the last test
    register_after_all_handler(module_path);

    // Re-throw any panic that occurred during the test
    if let Err(err) = result {
        panic::resume_unwind(err);
    }
}

/// Run before_all fixtures for a module if they haven't been run yet
fn run_before_all_if_needed(module_path: &'static str) {
    // Check if we've already executed the before_all fixtures for this module
    let mut executed = EXECUTED_MODULES.lock().unwrap();
    if !executed.contains(module_path) {
        // Mark as executed first to prevent potential infinite recursion
        executed.insert(module_path);

        // Run before_all fixtures
        if let Ok(fixtures) = BEFORE_ALL_FIXTURES.lock()
            && let Some(before_all_funcs) = fixtures.get(module_path)
        {
            for before_fn in before_all_funcs {
                before_fn();
            }
        }
    }
}

/// Register after_all fixtures to be run at process exit
fn register_after_all_handler(module_path: &'static str) {
    // We use ctor's dtor to register a function that will run at process exit
    // This is a bit of a hack, but it's the best we can do without modifying the test runner
    // The actual registration happens in the macro

    // Here we just ensure the module path is saved for the handler
    let mut executed = EXECUTED_MODULES.lock().unwrap();
    executed.insert(module_path);
}

/// Run all after_all fixtures that have been registered
/// This is called by an exit handler registered by the test runner
#[doc(hidden)]
pub fn run_after_all_fixtures() {
    // Get the list of modules that have been executed
    let executed = EXECUTED_MODULES.lock().unwrap();

    // Run after_all fixtures for each executed module
    if let Ok(fixtures) = AFTER_ALL_FIXTURES.lock() {
        for module_path in executed.iter() {
            if let Some(after_all_funcs) = fixtures.get(module_path) {
                for after_fn in after_all_funcs {
                    after_fn();
                }
            }
        }
    }
}

/// Check if we're running inside a fixture-wrapped test
pub fn is_in_fixture_test() -> bool {
    return IN_FIXTURE_TEST.with(|flag| *flag.borrow());
}
