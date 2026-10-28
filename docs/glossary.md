# Glossary

## Crate

A crate is a unit of compilation in Rust. For example, `src/main.rs` is the crate root of a binary crate. A crate can contain multiple modules, which may be spread across multiple files. A package can contain multiple binary crates.

### Binary Crate

A binary crate compiles into an executable program and must have a `main` function. A package can contain multiple binary crates, each with its own `main` function.

### Library Crate

A library crate provides functionality for other code to use and does not have a `main` function. A package can contain at most one library crate.

## Package

A package is a collection of one or more crates managed by Cargo. It contains a `Cargo.toml` file.

A package:

- must contain at least one crate
- can contain multiple binary crates
- can contain at most one library crate

## Associated Function

A function that belongs to a type, does not take `self`, and is called using `::`, for example, `String::new()`.

## Statement

A statement is an instruction that performs some action and does not return a value.

## Expression

An expression evaluates to a value.

## Unit Type

The unit type `()` represents the absence of a meaningful value. A function that does not return a meaningful value implicitly returns `()`.

## Stack

A region of memory that stores values in last-in, first-out (LIFO) order. Data stored on the stack has a known, fixed size.

## Heap

A region of memory used for dynamically allocated data. Data on the heap is accessed through a pointer.

## Borrowing

Borrowing allows code to access a value through a reference without taking ownership of it.

## Reference

A reference allows access to a value without taking ownership. References are created using `&`, and mutable references use `&mut`.

## Struct

A struct is a custom data type that groups related values together.

## Enum

An enum defines a type by listing its possible variants.

## Variant

A variant is one possible value of an enum. Each variant can store different types and amounts of data.

## Method

A method is a function defined inside an `impl` block that takes `self`, `&self`, or `&mut self` as its first parameter.

## Pattern Matching

Pattern matching compares a value against patterns and can destructure data stored inside enums and other types.

## Module

A module organizes related code inside a crate. Modules can be nested and can be defined in separate files.

## Generic Type Parameter

A placeholder for a concrete type, such as `T` in `Point<T>`.

Generics allow functions, structs, enums, and methods to work with different types without duplicating code.

## Trait

A trait defines shared behavior through methods. A type implements a trait using `impl Trait for Type`.

## Trait Bound

A constraint that requires a type to implement a specific trait.

For example, `T: Summary` allows generic code to use methods defined by `Summary`.

## Blanket Implementation

A trait implementation that applies to every type satisfying certain trait bounds.

For example, `impl<T: Display> Describe for T` implements `Describe` for all types that implement `Display`.
