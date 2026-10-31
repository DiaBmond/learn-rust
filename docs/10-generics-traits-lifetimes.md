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

## Lifetimes and the Borrow Checker

Every reference in Rust has a lifetime, which describes when that reference is valid.

The borrow checker verifies that references remain valid and prevents dangling references at compile time.

Lifetime annotations describe relationships between references. They do not extend the lifetime of the referenced data.

## Lifetime Relationships

A generic lifetime parameter, such as `'a`, expresses constraints between input and output references.

For example, `longest<'a>(x: &'a str, y: &'a str) -> &'a str` ties the returned reference to both inputs.

- The returned reference can only be used while both input borrows remain valid.
- `'a` does not force the input values to live for the same amount of time.
- The compiler checks these constraints at the call site.

A function can also tie its output to only one input.

For example, `first<'a>(x: &'a str, _y: &str) -> &'a str` returns a reference associated with `x`, independently of `y`.

Returning an owned `String` transfers ownership to the caller and does not require an output lifetime annotation.

A function cannot return a reference to a local `String` that is dropped when the function returns.

## Lifetime Annotations in Structs and Methods

A struct containing references must declare lifetime parameters for those references.

For example, `ImportantExcerpt<'a>` can contain a field `part: &'a str`.

- The referenced data must remain valid whenever the struct's reference is used.
- `impl<'a> ImportantExcerpt<'a>` defines methods for the struct.
- Methods returning owned values, such as `i32`, do not need output lifetime annotations.

For methods with `&self`, lifetime elision can associate an output reference with the borrow of `self`.

For example, `announce_and_return_part(&self, announcement: &str) -> &str` returns a reference tied to `self`, not to `announcement`.

## Lifetime Elision Rules

Rust can infer lifetimes in many function and method signatures using three lifetime elision rules.

- **Rule 1:** Each reference parameter receives its own lifetime parameter.
- **Rule 2:** If there is exactly one input lifetime, that lifetime is assigned to all elided output lifetimes.
- **Rule 3:** If a method has `&self` or `&mut self`, the lifetime of `self` is assigned to all elided output lifetimes.

For example, `first_word(s: &str) -> &str` works without explicit lifetime annotations because Rule 2 applies.

The same rule applies to `identity(s: &str, _number: usize) -> &str`, because `usize` does not introduce another input lifetime.

However, `choose(x: &str, y: &str) -> &str` requires an explicit lifetime annotation because the compiler cannot determine the output lifetime using these rules.

## The `'static` Lifetime

The `'static` lifetime can last for the entire duration of a program.

String literals have the type `&'static str` because their underlying data remains valid throughout the program.

- A reference with the `'static` lifetime can refer to data valid for the entire program.
- A local variable holding a `&'static str` can still go out of scope.
- Annotating a reference with `'static` does not extend the lifetime of a local value.

For example, a function cannot return `&'static str` borrowed from a local `String` that is dropped when the function returns.

## Combining Generics, Traits, and Lifetimes

A function can use generic lifetime parameters, generic type parameters, and trait bounds together.

For example, `longest_with_an_announcement<'a, T>` combines these concepts:

- `'a` is a generic lifetime parameter.
- `T` is a generic type parameter.
- `T: Display` requires the announcement type to implement `Display`.
- The returned `&'a str` is tied to the input string references, not to the announcement.

The same function can accept different announcement types, such as `&str` and `i32`, as long as they implement `Display`.