# Chapter 5: Structs and Methods

## Method vs Associated Function

Use a method when an operation needs an existing instance of the type.

```rust
rectangle.area();
rectangle.can_hold(&other);
```

Use an associated function when an operation belongs to the type but does not need an existing instance.

```rust
Rectangle::square(20);
String::new();
```

A useful rule of thumb:

- Needs `self` -> method
- Does not need `self` -> associated function
