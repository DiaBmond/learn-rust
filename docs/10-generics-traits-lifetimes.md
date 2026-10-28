# Chapter 10: Generics, Traits, and Lifetimes

## Generic Types and Methods

Generics allow the same code to work with different concrete types.

- `Point<T>` requires `x` and `y` to have the same type.
- `Point<X, Y>` allows `x` and `y` to have different types.
- `impl<T> Point<T>` defines methods available for every `Point<T>`.
- `impl Point<f32>` defines methods available only for `Point<f32>`.

Generic functions can also use trait bounds to require specific behavior from a type, such as `T: PartialOrd` for comparisons.

## Monomorphization

Rust uses monomorphization to generate specialized code for concrete types at compile time.

For example, calling `largest<T>()` with `i32` and `char` creates specialized versions for each type.

Conceptually:

- `largest::<i32>()` -> specialized for `i32`
- `largest::<char>()` -> specialized for `char`

This avoids runtime type checks for generic type parameters and provides performance comparable to writing separate functions manually.

The compiler may further optimize or inline the generated code. Monomorphization can increase compilation time and binary size.

## Required and Default Trait Methods

A trait method without a body must be implemented by each type.

A trait method with a body provides a default implementation that can be overridden.

A default method can call required methods from the same trait.

## `impl Trait` vs Generic Type Parameters

Both `&impl Summary` and `&T` with `T: Summary` accept values that implement `Summary`.

The difference becomes important with multiple parameters:

- Two `&impl Summary` parameters can have different concrete types.
- Two `&T` parameters sharing the same `T` must have the same concrete type.

## Multiple Trait Bounds and `where`

A generic type can require multiple traits using `+`.

For example, `T: Summary + Display` requires both traits.

A `where` clause expresses the same constraints and can make complex bounds easier to read.

## Returning `impl Trait`

A function returning `impl Summary` hides its concrete return type while guaranteeing that it implements `Summary`.

All return paths must use the same concrete type, even if multiple types implement the trait.

## Conditionally Implementing Methods

Methods can be available only when a generic type satisfies specific trait bounds.

- `impl<T> Pair<T>` defines methods available for every `Pair<T>`.
- `impl<T: Display + PartialOrd> Pair<T>` defines methods available only when `T` implements both traits.

A type can still use methods whose bounds it satisfies, even if other methods are unavailable.

## Blanket Implementations

A blanket implementation implements a trait for all types that satisfy a constraint.

For example, `impl<T: Display> Describe for T` makes `describe()` available to every type implementing `Display`.

The standard library uses the same idea to provide `ToString` for types implementing `Display`.

Unlike conditional methods, a blanket implementation implements a trait for many different types.
