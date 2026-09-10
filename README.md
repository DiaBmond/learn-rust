# Learn Rust

I'm learning Rust by following [The Rust Programming Language](https://doc.rust-lang.org/stable/book/).

## Glossary

### Crate

A crate is a unit of compilation in Rust. For example, `src/main.rs` is the crate root of a binary crate. A crate can contain multiple modules, which may be spread across multiple files. A package can contain multiple binary crates.

#### Binary Crate

A binary crate compiles into an executable program and must have a `main` function. A package can contain multiple binary crates, each with its own `main` function.

#### Library Crate

A library crate provides functionality for other code to use and does not have a `main` function. A package can contain at most one library crate.

### Package

A package is a collection of one or more crates managed by Cargo. It contains a `Cargo.toml` file.

A package:

- must contain at least one crate
- can contain multiple binary crates
- can contain at most one library crate

### Associated Function

A function that belongs to a type, does not take `self`, and is called using `::`, for example, `String::new()`.

### Statement

A statement is an instruction that performs some action and does not return a value.

### Expression

An expression evaluates to a value.

### Unit Type

The unit type `()` represents the absence of a meaningful value. A function that does not return a meaningful value implicitly returns `()`.

### Stack

A region of memory that stores values in last-in, first-out (LIFO) order. Data stored on the stack has a known, fixed size.

### Heap

A region of memory used for dynamically allocated data. Data on the heap is accessed through a pointer.

### Borrowing

Borrowing allows code to access a value through a reference without taking ownership of it.

### Reference

A reference allows access to a value without taking ownership. References are created using `&`, and mutable references use `&mut`.

### Struct

A struct is a custom data type that groups related values together.

### Enum

An enum defines a type by listing its possible variants.

### Variant

A variant is one possible value of an enum. Each variant can store different types and amounts of data.

### Method

A method is a function defined inside an `impl` block that takes `self`, `&self`, or `&mut self` as its first parameter.

### Pattern Matching

Pattern matching compares a value against patterns and can destructure data stored inside enums and other types.

## Concepts

### Constants vs Immutable Variables

In `03_variable_mutability`, I wondered why Rust has both immutable variables (`let x = ...`) and constants (`const X: Type = ...`) when neither can be changed.

- A constant is a fixed value known at compile time, while an immutable variable holds a value that cannot be changed after it is initialized.

### Ownership Rules

- Each value in Rust has an owner.
- There can be only one owner at a time.
- When the owner goes out of scope, the value is dropped.

### Move into an Inner Scope

When ownership is moved from a variable into another variable inside an inner scope, the original variable does not regain ownership when the inner scope ends.

```rust
let x = String::from("hello");

{
    let y = x; // ownership moves from `x` to `y`
    println!("{y}");
} // `y` goes out of scope and the String is dropped

// println!("{x}"); // Error: `x` no longer owns the String
```

### Borrowing Different Parts of the Same Value

I wondered whether an immutable slice and a mutable slice could exist at the same time if they refer to different parts of the same value.

```rust
let mut s = String::from("abcdef");

let a = &s[0..3];
let b = &mut s[4..6];

println!("{a}");
println!("{b}");
```

Even though the two ranges do not overlap, this does not compile. With normal slice indexing, Rust treats the borrows as overlapping and rejects them.

Conceptually, borrowing separate, non-overlapping parts can be safe, but Rust must be able to prove that the borrows do not overlap.

### Struct vs Enum

Use a struct when a value has the same set of fields every time.

Use an enum when a value can be one of several different variants.

For example:

- A `User` is naturally a struct because every user has the same kinds of fields, such as `username` and `email`.
- A `Message` is naturally an enum because a message can be `Quit`, `Move`, `Write`, or `ChangeColor`, and each variant can contain different data.

### Method vs Associated Function

Use a method when an operation needs an existing instance of the type.

```rust
rectangle.area();
rectangle.can_hold(&other);
```

Use an associated function when an operation belongs to the type but does not need an existing instance.

```rust
Rectangle::square(20);
String::new();
```

A useful rule of thumb:

- Needs `self` -> method
- Does not need `self` -> associated function

### Option

`Option<T>` represents a value that may or may not exist.

- `Some(value)` means a value is present.
- `None` means no value is present.

### Match Exhaustiveness

A `match` expression must handle every possible value. A catch-all pattern such as `_` can be used to handle any remaining cases.

### Catch-All Patterns

In a `match` expression:

- A named catch-all pattern, such as `other`, matches the remaining values and binds the matched value.
- `_` matches the remaining values without binding them.

### `if let` vs `let...else`

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

## Notes & Issues

For `02_hello_cargo`, I ran:

```bash
cargo new 02_hello_cargo
```

and got this error:

```bash
error: invalid character `0` in package name
```

Cargo uses the directory name as the package name by default, and package names cannot start with a number.

Instead, use:

```bash
cargo new 02_hello_cargo --name hello_cargo
```

This keeps the directory name as `02_hello_cargo` and sets the Cargo package name to `hello_cargo`.