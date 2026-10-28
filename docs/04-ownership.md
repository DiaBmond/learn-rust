# Chapter 4: Understanding Ownership

## Ownership Rules

- Each value in Rust has an owner.
- There can be only one owner at a time.
- When the owner goes out of scope, the value is dropped.

## Move into an Inner Scope

When ownership is moved from a variable into another variable inside an inner scope, the original variable does not regain ownership when the inner scope ends.

```rust
let x = String::from("hello");

{
    let y = x; // ownership moves from `x` to `y`
    println!("{y}");
} // `y` goes out of scope and the String is dropped

// println!("{x}"); // Error: `x` no longer owns the String
```

## Borrowing Different Parts of the Same Value

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

## Dereferencing

`*` accesses the value that a reference points to.

```rust
let mut number = 10;
let reference = &mut number;

*reference += 1;
```
