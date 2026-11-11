# Chapter 13: Functional Language Features

## Closures

Closures are anonymous functions that can capture values from their surrounding environment.

Unlike regular functions, closures can infer their parameter and return types from how they are used.

Closures can capture values in three ways:

- **Immutable borrowing (`&T`)** — reads captured values without modifying them.
- **Mutable borrowing (`&mut T`)** — modifies captured values.
- **Taking ownership (`T`)** — captures values by ownership, which can be forced using the `move` keyword.

The compiler determines how each value is captured based on how the closure uses it.

## Closure Traits

Rust automatically implements one or more closure traits based on how a closure uses its captured values:

- `FnOnce` — can be called at least once and may consume captured values.
- `FnMut` — can be called repeatedly and may mutate captured values.
- `Fn` — can be called repeatedly without mutating or consuming captured values.

Every closure implements `FnOnce`. Closures implementing `Fn` also implement `FnMut` and `FnOnce`.

These are traits available through the Rust prelude, not keywords.

## Iterators

An iterator produces a sequence of items one at a time.

All iterators implement the `Iterator` trait, which defines an associated type `Item` and the `next()` method.

- `next()` returns `Some(item)` while items remain.
- `next()` returns `None` when iteration is complete.
- Calling `next()` requires a mutable iterator because it advances the iterator's state.

Rust provides three common ways to create iterators:

- `iter()` — iterates over immutable references (`&T`).
- `iter_mut()` — iterates over mutable references (`&mut T`).
- `into_iter()` — consumes the collection and yields owned values (`T`).

Iterators are lazy: creating an iterator does not process its items until they are requested or consumed.

## Iterator Adapters

Iterator adapters transform or filter an iterator without immediately processing its items.

- `map()` — transforms each item using a closure.
- `filter()` — retains items that satisfy a condition.

Consuming methods drive iteration and produce results:

- `collect()` — gathers items into a collection.
- `sum()` — adds items together.
- `next()` — consumes one item at a time.

Iterator adapters can be chained to express data-processing operations without manually managing loops or intermediate collections.

## Improving the I/O Project

Iterators simplify the `minigrep` project from Chapter 12.

- `Config::build()` accepts an iterator of owned `String` values instead of a borrowed slice, eliminating unnecessary `clone()` calls.
- `env::args()` returns an iterator that can be passed directly to `Config::build()`.
- `search()` and `search_case_insensitive()` use `filter()` and `collect()` instead of manually pushing matching lines into a mutable vector.

Returning an iterator instead of a collected vector can also allow matching lines to be processed lazily.

## Performance and Zero-Cost Abstractions

Rust iterators and closures are designed as **zero-cost abstractions**, allowing high-level code without inherent runtime overhead from the abstraction itself.

Compiler optimizations such as inlining, loop unrolling, and bounds-check elimination can make iterator-based code perform similarly to explicit loops.

Neither approach is guaranteed to be faster in every situation. Choose the style that makes the code clearer, and benchmark when performance matters.