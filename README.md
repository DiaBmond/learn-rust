# Learn Rust

I'm learning Rust by following [The Rust Programming Language](https://doc.rust-lang.org/stable/book/).

## Glossary
### Crate
A crate is a unit of compilation in Rust. For example, `src/main.rs` is the crate root of a binary crate. A crate can contain multiple modules, which may be spread across multiple files. A package can contain multiple binary crates.

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