# Rest

[![Crates.io](https://img.shields.io/crates/v/rest.svg)](https://crates.io/crates/rest)
[![Build Status](https://github.com/mister-good-deal/rest/workflows/CI/badge.svg)](https://github.com/mister-good-deal/rest/actions)
[![codecov](https://codecov.io/gh/mister-good-deal/rest/graph/badge.svg?token=W5L8E2CQ9M)](https://codecov.io/gh/mister-good-deal/rest)
[![License](https://img.shields.io/crates/l/rest.svg)](https://github.com/mister-good-deal/rest/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/rest.svg)](https://crates.io/crates/rest)
[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/10418/badge)](https://www.bestpractices.dev/projects/10418)

A fluent, Jest-like testing library for Rust that builds upon the standard testing infrastructure. Rest provides
expressive assertions with readable error messages while maintaining compatibility with Rust's built-in testing functionality.

## Features

- **Fluent, Expressive API**: Write tests in a readable, chainable style similar to Jest.
- **Logical Chain Modifiers**: Combine assertions with `.and()` and `.or()` operators.
- **Test Fixtures**: Set up and tear down test environments with attribute-based fixtures.
- **Helpful Error Messages**: Get clear error messages that include variable names and expressions.
- **Seamless Integration**: Works alongside Rust's standard testing infrastructure.
- **Beautiful Test Output**: Enhanced test reporting with visual cues and better organization.
- **Type-Safe Assertions**: Leverages Rust's type system for compile-time safety.

## Roadmap

- [x] Basic matchers for primitive types
- [x] Matchers for collections (Vec, HashMap, etc.)
- [x] Matchers for Option and Result types
- [x] Logical chain modifiers (AND/OR)
- [x] Custom matcher support
- [x] Improved test output formatting
- [x] CI/CD integration for automatic releases
- [x] Documentation and examples
- [x] Test fixtures for setup and teardown

### v0.6.0 — Critical Fixes

- [ ] Generic numeric matchers (support all integer and float types, not just i32)
- [ ] Real regex matching in `to_match` (currently just uses `contains()`)
- [ ] Show actual vs expected values in error messages
- [ ] Remove unused/redundant dependencies (lazy_static, once_cell, thread_local)
- [ ] Fix plural conjugation bug for variable names ending in "s"

### v0.7.0 — New Matchers

- [ ] Float matchers with approximate equality (`to_be_close_to`)
- [ ] Panic matchers (`to_panic`, `to_panic_with`)
- [ ] Reduce code duplication in matcher helper traits
- [ ] Improve test coverage for ConsoleRenderer and modifiers

### v1.0.0 — Ecosystem

- [ ] Mocking and stubbing framework
- [ ] Diff output for complex value comparison (like pretty_assertions)
- [ ] Custom console client for enhanced terminal output
- [ ] Comprehensive documentation and API stabilization

## Quick Start

Add Rest to your project:

```bash
cargo add rest --dev
```

Write your first test:

```rust
use rest::prelude::*;

#[test]
fn should_check_values() {
    // By default, Rest behaves like standard Rust assertions
    // To enable enhanced output, configure it:
    config().enhanced_output(true).apply();
    
    let my_number = 5;
    let my_string = "hello world";
    let my_vec = vec![1, 2, 3];
    
    expect!(my_number).to_be_greater_than(3);
    expect!(my_string).to_contain("world");
    expect!(my_vec).to_have_length(3);
}
```

You can also enable enhanced output globally by setting the environment variable:

```bash
REST_ENHANCED_OUTPUT=true cargo test
```

## Available Matchers

Rest provides a comprehensive set of matchers for various types. All matchers support negation through either the
`not()` method or the `expect_not!` macro.

For complete documentation of all matchers, please see the [Wiki documentation](https://github.com/mister-good-deal/rest/wiki).

### Boolean Matchers

- **to_be_true** - Checks if a boolean is true
- **to_be_false** - Checks if a boolean is false

[View Boolean Matchers documentation](https://github.com/mister-good-deal/rest/wiki/Boolean-Matchers)

### Equality Matchers

- **to_equal** - Checks if a value equals another value

[View Equality Matchers documentation](https://github.com/mister-good-deal/rest/wiki/Equality-Matchers)

### Numeric Matchers

- **to_be_greater_than** - Checks if a number is greater than another
- **to_be_greater_than_or_equal** - Checks if a number is greater than or equal to another
- **to_be_less_than** - Checks if a number is less than another
- **to_be_less_than_or_equal** - Checks if a number is less than or equal to another
- **to_be_even** - Checks if a number is even
- **to_be_odd** - Checks if a number is odd
- **to_be_zero** - Checks if a number is zero
- **to_be_positive** - Checks if a number is positive
- **to_be_negative** - Checks if a number is negative
- **to_be_in_range** - Checks if a number is within a specified range

[View Numeric Matchers documentation](https://github.com/mister-good-deal/rest/wiki/Numeric-Matchers)

### String Matchers

- **to_be_empty** - Checks if a string is empty
- **to_contain** - Checks if a string contains a substring
- **to_start_with** - Checks if a string starts with a prefix
- **to_end_with** - Checks if a string ends with a suffix
- **to_match** - Checks if a string matches a pattern
- **to_have_length** - Checks if a string has a specific length

[View String Matchers documentation](https://github.com/mister-good-deal/rest/wiki/String-Matchers)

### Collection Matchers

- **to_be_empty** - Checks if a collection is empty
- **to_have_length** - Checks if a collection has a specific length
- **to_contain** - Checks if a collection contains a specific element
- **to_contain_all_of** - Checks if a collection contains all specified elements
- **to_equal_collection** - Compares two collections for element-wise equality

[View Collection Matchers documentation](https://github.com/mister-good-deal/rest/wiki/Collection-Matchers)

### HashMap Matchers

- **to_be_empty** - Checks if a HashMap is empty
- **to_have_length** - Checks if a HashMap has a specific length
- **to_contain_key** - Checks if a HashMap contains a specific key
- **to_contain_entry** - Checks if a HashMap contains a specific key-value pair

[View HashMap Matchers documentation](https://github.com/mister-good-deal/rest/wiki/HashMap-Matchers)

### Option Matchers

- **to_be_some** - Checks if an Option contains a value
- **to_be_none** - Checks if an Option is None
- **to_contain_value** - Checks if an Option contains a specific value

[View Option Matchers documentation](https://github.com/mister-good-deal/rest/wiki/Option-Matchers)

### Result Matchers

- **to_be_ok** - Checks if a Result is Ok
- **to_be_err** - Checks if a Result is Err
- **to_contain_ok** - Checks if a Result contains a specific Ok value
- **to_contain_err** - Checks if a Result contains a specific Err value

[View Result Matchers documentation](https://github.com/mister-good-deal/rest/wiki/Result-Matchers)

## Using Modifiers

Rest provides powerful modifiers to create complex assertions, including:

- Negation with the `.not()` method or `expect_not!` macro
- Logical chaining with `.and()` and `.or()` operators
- Combining negation with logical operators

```rust
// Example of chained assertions
expect!(number).to_be_greater_than(30)
             .and().to_be_less_than(50)
             .and().to_be_even();

// Example of negation
expect!(value).not().to_equal(100);
```

[View Using Modifiers documentation](https://github.com/mister-good-deal/rest/wiki/Using-Modifiers)

## Test Fixtures

Rest provides a powerful fixture system for setting up and tearing down test environments:

```rust
use rest::prelude::*;

// Define setup function
#[setup]
fn prepare_test_data() {
    // Code to run before each test
    println!("Setting up test environment");
}

// Define teardown function
#[tear_down]
fn cleanup_resources() {
    // Code to run after each test
    println!("Cleaning up test environment");
}

// Test with fixtures
#[test]
#[with_fixtures]
fn my_test() {
    // This will automatically run setup before and teardown after
    expect!(2 + 2).to_equal(4);
}
```

Key features:

- Complete test lifecycle management with `#[before_all]`, `#[setup]`, `#[tear_down]`, and `#[after_all]`
- Attribute-based API with `#[setup]`, `#[tear_down]`, and `#[with_fixtures]`
- Module-level fixtures with `#[with_fixtures_module]` to apply fixtures to all tests in a module
- Module-scoped fixtures (fixtures are tied to the module they're defined in)
- Automatic cleanup on test failures
- Multiple setup/teardown functions per module

[View Test Fixtures documentation](https://github.com/mister-good-deal/rest/wiki/Fixtures)

## Custom Matchers

Rest is designed to be easily extensible. You can create your own custom matchers to make your tests more expressive and domain-specific.

[View Custom Matchers documentation](https://github.com/mister-good-deal/rest/wiki/Custom-Matchers)

## Output Formatting

Rest enhances the standard test output with colors, symbols, and improved formatting:

- **Color Coding**: Green for passing tests, red for failing tests
- **Unicode Symbols**: Check (✓) marks for passing conditions, cross (✗) for failing ones
- **Clean Variable Names**: Reference symbols (`&`) are automatically removed from output
- **Consistent Indentation**: Multi-line output is properly indented for readability

[View Output Formatting documentation](https://github.com/mister-good-deal/rest/wiki/Output-Formatting)

## Architecture

Rest uses a modular, event-driven architecture:

- **Backend Layer** - Core assertion evaluation logic
- **Config System** - Controls the library's behavior
- **Event System** - Decouples assertion execution from reporting
- **Frontend Layer** - Reporting and user interface

[View Architecture documentation](https://github.com/mister-good-deal/rest/wiki/Architecture)

## Releases

This project is automatically published to [crates.io](https://crates.io/crates/rest) when:

1. The version in Cargo.toml is increased beyond the latest git tag
2. The code is merged to the master branch
3. All CI checks pass (tests, examples, linting)

The publishing workflow will:

1. Create a git tag for the new version (vX.Y.Z)
2. Publish the package to crates.io
3. Generate a GitHub release using notes from CHANGELOG.md
4. Fall back to auto-generated notes if no CHANGELOG entry exists

## Code Coverage

This project uses Rust's official LLVM-based code coverage instrumentation to track test coverage. The coverage workflow:

1. Compiles the project with coverage instrumentation using Rust nightly's `-C instrument-coverage` flag
2. Runs the test suite to generate raw coverage data in LLVM's profraw format
3. Uses grcov to convert the raw coverage data to:
   - LCOV format for uploading to Codecov.io
   - HTML reports for local viewing

Coverage reports are automatically generated on each push to the master branch and for pull requests.

### Viewing Coverage Reports

- The latest coverage report is always available on [Codecov.io](https://codecov.io/gh/mister-good-deal/rest)
- Each CI run also produces an HTML coverage report available as a downloadable artifact

### Running Code Coverage Locally

To generate coverage reports locally:

```bash
# Install required components
rustup component add llvm-tools-preview
cargo install grcov
cargo install rustfilt

# Build with coverage instrumentation and run tests
RUSTFLAGS="-C instrument-coverage" cargo test

# Generate HTML report
grcov . --binary-path ./target/debug/ -s . -t html --branch --keep-only "src/**" -o ./coverage

# Generate Markdown report
grcov . --binary-path ./target/debug/ -s . -t markdown --branch --keep-only "src/**" -o ./coverage/coverage.md
```

Then open `./coverage/index.html` in your browser or view the Markdown report in your favorite editor.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
