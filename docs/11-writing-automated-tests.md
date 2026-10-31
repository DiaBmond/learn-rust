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

*To be added.*

## 11.3 Test Organization

*To be added.*