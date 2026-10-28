# Chapter 9: Error Handling

## Error Handling

Rust provides two main approaches to error handling:

- `Result<T, E>` represents an operation that can succeed or fail, allowing the caller to decide how to handle errors.
- `panic!` stops normal execution when continuing would be inappropriate.

A useful rule of thumb:

- Use `Result` for expected failures, such as a missing file or invalid user input.
- Use `panic!` when an assumption or invariant is violated.

`unwrap()` and `expect()` extract the value from `Ok` but panic on `Err`. `expect()` also provides a custom error message.

## Propagating Errors with `?`

The `?` operator allows a function to return an error to its caller instead of handling it locally.

For `Result<T, E>`:

- `Ok(value)` -> extracts `value` and continues.
- `Err(error)` -> returns the error early.

For `Option<T>`:

- `Some(value)` -> extracts `value` and continues.
- `None` -> returns `None` early.

The surrounding function must have a compatible return type.

Unlike `unwrap()` and `expect()`, `?` does not panic when it encounters an error.

## Invariants and Validation

An invariant is a condition that must always remain true.

A custom type can protect an invariant by keeping its fields private and validating values during construction.

For example, a `Guess` type can ensure that its value is always between 1 and 100, so other functions do not need to repeat the same validation.
