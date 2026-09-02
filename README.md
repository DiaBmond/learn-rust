# Learn Rust

I'm learning Rust by following [The Rust Programming Language](https://doc.rust-lang.org/stable/book/).

## Glossary
### Crate
A crate is a unit of compilation in Rust. For example, `src/main.rs` is the crate root of a binary crate. A crate can contain multiple modules, which may be spread across multiple files. A package can contain multiple binary crates.
### Associated function
A function that belongs to a type and is called using `::`, for example, `String::new()`.
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

## Concepts
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