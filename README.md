# Learn Rust

I'm learning Rust by following [The Rust Programming Language](https://doc.rust-lang.org/stable/book/).

## Glossary
### Crate
A crate is a unit of compilation in Rust. For example, `src/main.rs` is the crate root of a binary crate. A crate can contain multiple modules, which may be spread across multiple files. A package can contain multiple binary crates.
### Associated function
A function that belongs to a type and is called using `::`, for example, `String::new()`.
### Statement
A statement is an instructions that perform some action and do not return a value.
### Expression
An expression evaluate to a resultant value.
### Unit Type
The unit type `()` represents the absence of a meaningful value. A function that does not return a meaningful value implicitly returns `()`.
### Stack
A region of memory that stores values in last-in, first-out (LIFO) order. Data stored on the stack has a known, fixed size.
### Heap
A region of memory used for dynamically allocated data. Data on the heap is accessed through a pointer.

## Concepts
In `03_variable_mutability`, I wondered why Rust has both immutable variables (`let x = ...`) and constants (`const X: Type = ...`) when neither can be changed.
- A constant is a fixed value known at compile time, while an immutable variable holds value that cannot be changed after it is initialized.

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