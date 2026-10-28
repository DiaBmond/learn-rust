# Chapter 6: Enums and Pattern Matching

## Struct vs Enum

Use a struct when a value has the same set of fields every time.

Use an enum when a value can be one of several different variants.

For example:

- A `User` is naturally a struct because every user has the same kinds of fields, such as `username` and `email`.
- A `Message` is naturally an enum because a message can be `Quit`, `Move`, `Write`, or `ChangeColor`, and each variant can contain different data.

## Option

`Option<T>` represents a value that may or may not exist.

- `Some(value)` means a value is present.
- `None` means no value is present.

## Match Exhaustiveness

A `match` expression must handle every possible value. A catch-all pattern such as `_` can be used to handle any remaining cases.

## Catch-All Patterns

In a `match` expression:

- A named catch-all pattern, such as `other`, matches the remaining values and binds the matched value.
- `_` matches the remaining values without binding them.

## `if let` vs `let...else`

Use `if let` when some code should run only if a value matches a pattern.

```rust
if let Some(value) = option {
    println!("{value}");
}
```

If the pattern does not match, execution simply continues after the `if let`.

Use `let...else` when a value must match a pattern in order for the rest of the function or block to continue.

```rust
let Some(value) = option else {
    return;
};

println!("{value}");
```

The `else` branch must diverge, for example, by using `return`, `break`, or `panic!`.

A useful rule of thumb:

- `if let` -> "If it matches, do this."
- `let...else` -> "It must match, otherwise leave."
