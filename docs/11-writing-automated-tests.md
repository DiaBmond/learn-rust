# Chapter 11: Writing Automated Tests

Automated tests help verify that Rust programs behave as expected. They detect incorrect behavior, document requirements, and help prevent regressions when code changes.

## 11.1 How to Write Tests

### Structuring Test Functions

Rust provides a built-in test framework through Cargo.

Test functions are marked with the `#[test]` attribute and are commonly placed inside a `tests` module.

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_works() {
        assert_eq!(add(2, 2), 4);
    }
}
```

- `#[cfg(test)]` includes the module when compiling the crate for unit testing.
- `#[test]` marks a function as a test.
- `use super::*;` brings items from the parent module into scope.
- `cargo test` compiles and executes the test suite.

A normal test passes when it finishes without panicking and its assertions succeed.

### Checking Conditions with `assert!`

The `assert!` macro verifies that a Boolean expression evaluates to `true`.

```rust
assert!(larger.can_hold(&smaller));
assert!(!smaller.can_hold(&larger));
```

If the condition evaluates to `false`, the macro panics and the test fails.

The `Rectangle` exercises demonstrate testing both expected success and expected rejection.

Boundary cases are also important. For example, two rectangles with identical dimensions cannot hold each other when the implementation uses strict `>` comparisons.

### Testing Equality with `assert_eq!` and `assert_ne!`

The standard library provides macros for comparing values.

```rust
assert_eq!(add_two(2), 4);
assert_ne!(add_two(10), 10);
```

- `assert_eq!` passes when both values are equal.
- `assert_ne!` passes when the values are different.
- When an assertion fails, these macros display the compared values using debug formatting.

The compared types must implement `PartialEq` and `Debug`.

Custom types can derive these traits:

```rust
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

Unlike `assert!(left == right)`, `assert_eq!` automatically displays both values when the comparison fails.

### Adding Custom Failure Messages

Assertions can include custom messages to make failures easier to understand.

```rust
let name = "Carol";
let result = greeting(name);

assert!(
    result.contains(name),
    "Expected greeting to contain `{name}`, but got `{result}`"
);
```

Custom messages use the same formatting syntax as `format!`.

They are displayed only when an assertion fails.

Tests should verify the intended requirements rather than unnecessary implementation details. For example, checking that a greeting contains a name allows the greeting format to change without breaking the test.

### Checking for Panics with `#[should_panic]`

Some functions are designed to panic when their preconditions are violated.

The `#[should_panic]` attribute verifies that a test causes a panic.

```rust
#[test]
#[should_panic]
fn guess_above_100_panics() {
    Guess::new(200);
}
```

A test marked with `#[should_panic]` passes when a panic occurs and fails when no panic occurs.

However, it may pass because of an unrelated panic.

The `expected` parameter makes the test more precise by checking the panic message.

```rust
#[test]
#[should_panic(expected = "less than or equal to 100")]
fn guess_above_100_panics_with_correct_message() {
    Guess::new(200);
}
```

The expected message is matched as a substring, not necessarily as an exact match.

The `Guess` exercises verify:

- Values below 1 cause a panic.
- Values above 100 cause a panic.
- Panic messages identify the correct invalid condition.
- Boundary values 1 and 100 are accepted.

### Using `Result<T, E>` in Tests

Test functions can return `Result<(), E>` instead of relying exclusively on assertions.

```rust
#[test]
fn addition_returns_ok() -> Result<(), String> {
    let result = add(2, 2);

    if result == 4 {
        Ok(())
    } else {
        Err(format!("Expected 4, but got {result}"))
    }
}
```

- `Ok(())` indicates success.
- `Err(error)` indicates failure.
- Returning `Err` does not itself cause a panic.

Returning `Result` also allows the use of the `?` operator.

```rust
#[test]
fn valid_number_parses_successfully(
) -> Result<(), std::num::ParseIntError> {
    let number: i32 = "42".parse()?;

    assert_eq!(number, 42);

    Ok(())
}
```

If parsing returns `Err`, the `?` operator immediately propagates the error and the test fails.

When testing an operation that is expected to return an error, inspect the `Result` directly.

```rust
#[test]
fn invalid_number_returns_error() {
    let result = "hello".parse::<i32>();

    assert!(result.is_err());
}
```

Do not use `?` when the purpose of the test is to confirm that an operation returns `Err`.

### Ignoring Tests for Experimentation

The `#[ignore]` attribute skips a test during a normal test run.

```rust
#[test]
#[ignore = "Practice reading assert_eq! failure output"]
fn demo_assert_eq_failure() {
    assert_eq!(4, 5);
}
```

The optional string documents why the test is ignored.

Ignored tests are skipped, not passed.

This is useful for keeping intentionally failing examples in the project without breaking the normal test suite.

The `#[ignore]` attribute and test filtering will be explored further in Chapter 11.2.

### Running the Exercises

Run all non-ignored tests:

```bash
cargo test
```

Run a specific test:

```bash
cargo test addition_works
```

Run an intentionally failing test:

```bash
cargo test demo_assert_eq_failure -- --ignored
```

Experiment with different failure scenarios:

```bash
cargo test demo_assert_failure -- --ignored
cargo test demo_custom_message_failure -- --ignored
cargo test demo_wrong_panic_message -- --ignored
cargo test demo_missing_panic -- --ignored
cargo test demo_result_error -- --ignored
cargo test demo_question_mark_error -- --ignored
```

Run only ignored tests:

```bash
cargo test -- --ignored
```

Run all tests, including ignored tests:

```bash
cargo test -- --include-ignored
```

The intentionally failing tests are expected to fail when executed.

### Key Takeaways

- `#[test]` identifies test functions.
- `assert!` checks Boolean conditions.
- `assert_eq!` and `assert_ne!` compare values.
- Custom failure messages improve debugging.
- `#[should_panic]` verifies expected panics.
- `expected` checks for a substring in the panic message.
- Tests returning `Result<(), E>` can propagate errors with `?`.
- `#[ignore]` skips tests until explicitly requested.
- Tests should cover both valid and invalid inputs, including boundary cases.
- Passing tests confirm only the behaviors covered by the test cases.

## 11.2 Controlling How Tests Are Run

The `cargo test` command compiles a crate in test mode and runs the resulting test binary.

By default, Rust runs tests in parallel and captures their standard output to keep test results readable.

Test execution can be customized using command-line arguments.

Some arguments belong to Cargo, while others belong to the test binary. The `--` separator distinguishes them.

```bash
cargo test [cargo-options] -- [test-options]
```

For example:

```bash
cargo test --release -- --test-threads=1
```

- `--release` is a Cargo option that uses the release profile.
- `--` separates Cargo arguments from test binary arguments.
- `--test-threads=1` is a test harness option that runs tests sequentially.

The available options can be inspected with:

```bash
cargo test --help
cargo test -- --help
```

The first command displays Cargo options, while the second displays test binary options.

### Running Tests in Parallel or Consecutively

By default, Rust runs multiple tests in parallel using threads.

Parallel execution can reduce testing time, but tests should not depend on one another or interfere with shared resources.

Shared resources may include:

- Files on disk.
- Environment variables.
- The current working directory.
- Shared mutable state.

For example, two tests that write different values to the same file may interfere with each other. One test could overwrite the file before another test reads it, causing an unexpected failure.

This behavior can produce a race condition or a flaky test.

To run tests sequentially, set the number of test threads to `1`:

```bash
cargo test -- --test-threads=1
```

To use two test threads:

```bash
cargo test -- --test-threads=2
```

The exercises include two tests that simulate work using `thread::sleep`:

```rust
#[test]
fn demo_parallel_task_a() {
    println!("Task A: started");

    thread::sleep(Duration::from_millis(200));

    println!("Task A: finished");

    assert_eq!(add(2, 2), 4);
}

#[test]
fn demo_parallel_task_b() {
    println!("Task B: started");

    thread::sleep(Duration::from_millis(200));

    println!("Task B: finished");

    assert_eq!(add_two(2), 4);
}
```

Run both tests with two test threads:

```bash
cargo test demo_parallel_task -- --test-threads=2 --nocapture
```

Run them sequentially:

```bash
cargo test demo_parallel_task -- --test-threads=1 --nocapture
```

With two threads, the tests may execute at the same time. With one thread, the test harness executes one test at a time.

Sequential execution does not guarantee a particular test order. It also does not automatically prevent problems caused by shared state that is not properly reset.

Tests should ideally be independent and use separate resources.

### Showing Function Output

By default, Rust captures standard output produced by tests.

If a test passes, its output from `println!` is hidden. If a test fails, the captured output is displayed with the failure details.

The exercises use a function that prints its argument and always returns `10`:

```rust
fn prints_and_returns_10(value: i32) -> i32 {
    println!("I got the value {value}");
    10
}
```

Two tests demonstrate successful and failing assertions:

```rust
#[test]
fn demo_prints_and_returns_10_pass() {
    let value = prints_and_returns_10(4);

    assert_eq!(value, 10);
}

#[test]
#[ignore = "Practice inspecting captured output from a failing test"]
fn demo_prints_and_returns_10_fail() {
    let value = prints_and_returns_10(8);

    assert_eq!(value, 5);
}
```

The passing test prints `I got the value 4`, but the message is hidden during a normal test run.

To display captured output from successful tests, use `--show-output`:

```bash
cargo test demo_prints_and_returns_10 -- --show-output
```

The test harness displays the captured message in a `successes` section.

To run both tests, including the intentionally failing test:

```bash
cargo test demo_prints_and_returns_10 -- --include-ignored --show-output
```

The failing test displays `I got the value 8` along with the assertion failure because the function returns `10`, not `5`.

The `--nocapture` option disables output capturing:

```bash
cargo test demo_prints_and_returns_10 -- --nocapture
```

Unlike `--show-output`, which displays captured output in the test summary, `--nocapture` allows output to appear during test execution.

When several tests print concurrently, their output may be interleaved. Running tests sequentially can make the output easier to follow.

```bash
cargo test -- --test-threads=1 --nocapture
```

### Running a Subset of Tests by Name

A large test suite may take a long time to execute.

Cargo allows tests to be selected by passing a name filter to `cargo test`.

The exercises include three tests for `add_two`:

```rust
#[test]
fn add_two_and_two() {
    let result = add_two(2);

    assert_eq!(result, 4);
}

#[test]
fn add_three_and_two() {
    let result = add_two(3);

    assert_eq!(result, 5);
}

#[test]
fn one_hundred() {
    let result = add_two(100);

    assert_eq!(result, 102);
}
```

#### Running Single Tests

To run the test named `one_hundred`:

```bash
cargo test one_hundred
```

Only tests matching the provided filter are executed.

Other tests are reported as `filtered out`.

A name filter is not a list of separate test names. To run several tests, use a shared substring or module path.

#### Filtering to Run Multiple Tests

The test filter matches a substring of the test's full name.

For example:

```bash
cargo test add
```

This command runs tests whose names contain `add`.

In the exercises, matching tests include:

- `addition_works`
- `add_two_returns_expected_value`
- `add_two_changes_original_value`
- `add_two_and_two`
- `add_three_and_two`

The `one_hundred` test does not match `add`, even though it calls the `add_two` function.

Filtering depends on the test name, not on which functions the test calls.

Module paths are also part of test names.

For example, a test defined inside `mod tests` has a full name such as:

```text
tests::addition_works
```

Tests can therefore be filtered by module path:

```bash
cargo test tests::
```

To list available tests without executing them:

```bash
cargo test -- --list
```

### Ignoring Tests Unless Specifically Requested

Some tests may be slow, expensive, or intentionally designed to fail.

The `#[ignore]` attribute excludes a test from normal test execution.

```rust
#[test]
#[ignore = "Practice reading assert_eq! failure output"]
fn demo_assert_eq_failure() {
    assert_eq!(add_two(2), 5);
}
```

The optional string explains why the test is ignored.

An ignored test is skipped rather than passed or failed.

Run all non-ignored tests:

```bash
cargo test
```

Run only ignored tests:

```bash
cargo test -- --ignored
```

Run all tests, including ignored tests:

```bash
cargo test -- --include-ignored
```

To execute one specific ignored test:

```bash
cargo test demo_assert_eq_failure -- --ignored
```

The exercises contain several intentionally failing tests marked with `#[ignore]`.

Running all ignored tests or including ignored tests in the normal suite will therefore produce expected failures.

The distinction between the main test result categories is:

- `passed`: The test executed successfully.
- `failed`: The test executed but did not satisfy its expected behavior.
- `ignored`: The test was skipped because it was marked as ignored.
- `filtered out`: The test did not match the selected name filter or other selection criteria.

### Running the Exercises

Run the normal test suite:

```bash
cargo test
```

Run tests sequentially:

```bash
cargo test -- --test-threads=1
```

Compare parallel and sequential execution:

```bash
cargo test demo_parallel_task -- --test-threads=2 --nocapture
cargo test demo_parallel_task -- --test-threads=1 --nocapture
```

Display captured output from passing tests:

```bash
cargo test demo_output_capture -- --show-output
cargo test demo_prints_and_returns_10 -- --show-output
```

Run the passing and intentionally failing output tests:

```bash
cargo test demo_prints_and_returns_10 -- --include-ignored --show-output
```

Run one test by name:

```bash
cargo test one_hundred
```

Filter tests using a substring:

```bash
cargo test add
```

List available tests:

```bash
cargo test -- --list
```

Run a specific ignored test:

```bash
cargo test demo_assert_eq_failure -- --ignored
```

The intentionally failing exercises are expected to fail when explicitly executed.

### Key Takeaways

- `cargo test` compiles and executes Rust tests.
- The `--` separator distinguishes Cargo options from test binary options.
- Tests run in parallel by default.
- `--test-threads=1` runs tests sequentially.
- Tests should not depend on execution order or shared mutable state.
- Rust captures standard output from tests by default.
- `--show-output` displays captured output from passing tests.
- `--nocapture` disables output capturing.
- `cargo test name` filters tests by their names.
- A substring can match multiple test names.
- Module paths are included in full test names.
- `#[ignore]` excludes tests from normal test runs.
- `--ignored` runs only ignored tests.
- `--include-ignored` runs both normal and ignored tests.
- Independent tests are more reliable and easier to maintain.

## 11.3 Test Organization

*To be added.*