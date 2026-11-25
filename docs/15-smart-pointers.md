
# Chapter 15: Smart Pointers

Smart pointers are data structures that behave like pointers while providing additional functionality, such as ownership management, reference counting, and automatic cleanup.

Unlike regular references, smart pointers often own the data they point to. Many smart pointers implement the `Deref` and `Drop` traits.

## Box<T>

`Box<T>` stores data on the heap while maintaining single ownership.

```rust
let b = Box::new(5);
println!("{b}");
```

Common reasons to use `Box<T>`:

- Store data on the heap.
- Define recursive types with a known size.
- Transfer ownership without copying large amounts of data.
- Own values through trait objects.

### Recursive Types

Recursive types require indirection because their size cannot otherwise be determined at compile time.

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

`Box<List>` has a known size regardless of the length of the list.

## Deref Trait

The `Deref` trait allows a type to behave like a reference through the dereference operator `*`.

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

For a custom type implementing `Deref`, the expression `*value` conceptually becomes `*(value.deref())`.

### Deref Coercion

Deref coercion automatically converts references between compatible types when calling functions or methods.

For example, `&MyBox<String>` can be coerced into `&String` and then into `&str`.

Rust supports deref coercion in three cases:

- `&T` to `&U` when `T` implements `Deref<Target = U>`.
- `&mut T` to `&mut U` when `T` implements `DerefMut<Target = U>`.
- `&mut T` to `&U` when `T` implements `Deref<Target = U>`.

An immutable reference cannot be coerced into a mutable reference.

## Drop Trait

The `Drop` trait allows a type to perform custom cleanup when its value is dropped.

```rust
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Cleaning up resources");
    }
}
```

Local variables are normally dropped in reverse order of their declaration when they leave scope.

The `Drop::drop` method cannot be called directly. Use the `drop()` function to release a value early:

```rust
let value = Box::new(5);
drop(value);
```

The `drop()` function takes ownership of the value, making it unavailable afterward.

## Rc<T>

`Rc<T>` is a reference-counted smart pointer that enables multiple owners of the same data.

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a);

assert_eq!(2, Rc::strong_count(&a));
```

- `Rc::clone()` creates another owner without deeply copying the inner value.
- `Rc::strong_count()` returns the number of strong owners.
- Dropping an owner decreases the strong reference count.
- The inner value is dropped when the strong count reaches zero.

`Rc<T>` provides shared access to data and is intended for single-threaded scenarios.

## RefCell<T> and Interior Mutability

Interior mutability allows data to be modified through a shared reference.

`RefCell<T>` enables interior mutability by enforcing borrowing rules at runtime instead of compile time.

```rust
use std::cell::RefCell;

let value = RefCell::new(5);

*value.borrow_mut() += 10;

assert_eq!(15, *value.borrow());
```

### Runtime Borrow Checking

`RefCell<T>` provides two methods for borrowing its inner value:

- `borrow()` returns `Ref<T>` for immutable access.
- `borrow_mut()` returns `RefMut<T>` for mutable access.

The borrowing rules still apply:

- Multiple immutable borrows are allowed.
- Only one mutable borrow is allowed at a time.
- Mutable and immutable borrows cannot coexist.

Violating these rules causes a runtime panic.

Runtime checking introduces a small performance overhead but allows patterns that compile-time borrowing checks cannot verify.

### Testing with Mock Objects

A mock object records interactions during a test so that expected behavior can be verified.

`RefCell<T>` allows a mock object to record messages inside a method that takes `&self`, without changing the original trait interface.

```rust
struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages
            .borrow_mut()
            .push(String::from(message));
    }
}
```

### Shared Mutable Data

Combining `Rc<T>` with `RefCell<T>` allows multiple owners to modify shared data.

```rust
let a = Rc::new(RefCell::new(5));
let b = Rc::clone(&a);

*b.borrow_mut() += 10;

assert_eq!(15, *a.borrow());
```

`Rc<T>` manages shared ownership, while `RefCell<T>` manages runtime borrowing.

Both types are intended for single-threaded use when sharing data.

## Reference Cycles and Memory Leaks

A reference cycle occurs when values directly or indirectly own each other through strong references.

Combining `Rc<T>` with `RefCell<T>` can create cycles because references between values can be modified after creation.

```text
a --Rc--> b
a <--Rc-- b
```

The strong reference counts never reach zero, preventing the values from being dropped.

Rust guarantees memory safety but does not guarantee the absence of memory leaks.

### Weak<T>

`Weak<T>` is a non-owning reference to a value managed by `Rc<T>`.

```rust
let strong = Rc::new(5);
let weak = Rc::downgrade(&strong);

assert_eq!(1, Rc::strong_count(&strong));
assert_eq!(1, Rc::weak_count(&strong));

drop(strong);

assert!(weak.upgrade().is_none());
```

- `Rc::downgrade()` creates a weak reference without increasing the strong count.
- `Rc::weak_count()` returns the number of weak references.
- `Weak::upgrade()` returns `Option<Rc<T>>`.
- Weak references do not prevent the inner value from being dropped.

### Preventing Reference Cycles

A tree can use strong references for child nodes and weak references for parent nodes.

```rust
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

A parent owns its children, while a child references its parent without owning it.

This allows bidirectional relationships without creating a strong ownership cycle.

## Smart Pointer Comparison

| Type | Ownership | Main Purpose |
| --- | --- | --- |
| `Box<T>` | Single | Heap allocation |
| `Rc<T>` | Shared | Reference-counted ownership |
| `RefCell<T>` | Single | Interior mutability |
| `Rc<RefCell<T>>` | Shared | Shared mutable data |
| `Weak<T>` | Non-owning | Access without extending the value's lifetime |

`Box<T>` uses compile-time borrow checking. `RefCell<T>` checks borrows of its inner value at runtime.

## Key Takeaways

Smart pointers provide different ownership, borrowing, and resource management guarantees.

The `Deref` trait enables reference-like access, while the `Drop` trait enables automatic cleanup.

Choose smart pointers based on ownership relationships, mutation requirements, and value lifetimes. Prefer regular references when they are sufficient, and use weak references to prevent strong ownership cycles.
