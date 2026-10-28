# Chapter 8: Common Collections

## Accessing Vector Elements

- `&v[index]` returns `&T` and panics if the index is out of bounds.
- `v.get(index)` returns `Option<&T>` and returns `None` if the index is out of bounds.

## Borrowing and Growing a Vector

`push` may reallocate a vector and move its elements, so Rust prevents mutation while a reference to an element is still in use.

A borrow can end after its last use; it does not always last until the end of the scope.

## Using Enums with Vectors

A vector stores values of one type. An enum can wrap different kinds of data into a single type.

For example, `Vec<SpreadsheetCell>` can contain `Int(i32)`, `Float(f64)`, and `Text(String)` variants while every element is still a `SpreadsheetCell`.

## Strings and UTF-8

Rust strings are UTF-8 encoded.

- `String::len()` returns the number of bytes.
- Strings cannot be indexed with `s[index]`.
- String slices use byte ranges and must be on valid character boundaries.
- `.chars()` iterates over characters, while `.bytes()` iterates over bytes.

## Updating HashMap Values with `entry`

`entry(key).or_insert(value)` inserts a value if the key is missing and returns a mutable reference to the stored value.

```rust
let count = map.entry(key).or_insert(0);
*count += 1;
```

## HashMap Iteration Order

A `HashMap` does not guarantee iteration order. If deterministic ordering is needed, collect the values and sort them separately.
