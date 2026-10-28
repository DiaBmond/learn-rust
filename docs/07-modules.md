# Chapter 7: Modules and Packages

## `pub` with Structs vs Enums

- `pub struct` does not make its fields public; fields need their own `pub`.
- `pub enum` makes all of its variants public automatically.

## `mod` vs `use`

- `mod` declares or defines a module.
- `use` brings a path into scope so it can be referenced more conveniently.
- `pub use` brings an item into scope and re-exports it as part of the public API.
