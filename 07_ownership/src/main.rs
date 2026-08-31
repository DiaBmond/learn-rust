fn main() {
    println!("***OWNERSHIP***");
    variable_scope();
    string_type();
    copy_vs_move();
    scope_and_assignment();
    clone_string();
    ownership_and_functions();
    return_values_and_scope();
    return_multiple_values();
}

fn variable_scope() {
    println!("== Variable Scope ==");

    let s = "outer scope";
    println!("{s}");

    {
        let s = "inner scope";
        println!("{s}");
    }

    println!("{s}");

    println!();
}

fn string_type() {
    println!("== The String Type ==");

    let mut s = String::from("hello"); // type: String

    s.push_str(", world!"); // appends a string slice (&str) to the String

    println!("{s}"); // prints `hello, world!`

    let literal = "hello"; // string literal, type: &str
    println!("{literal}");

    println!();
}

fn copy_vs_move() {
    println!("== Copy vs Move ==");

    // `i32` implements the `Copy` trait,
    // so the value is copied instead of moved.
    let x = 5;
    let y = x;

    println!("x: {x}, y: {y}"); // both are still valid

    // `String` does not implement `Copy`,
    // so ownership moves from `s1` to `s2`.
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("s1: {s1}"); // Error: `s1` was moved
    println!("s2: {s2}");

    println!();
}

fn scope_and_assignment() {
    println!("== Scope and Assignment ==");

    let mut s = String::from("hello");
    println!("{s}, world!");

    s = String::from("ahoy"); // the old String `"hello"` is dropped and replaced with `"ahoy"`

    println!("{s}, world!");

    println!();
}

fn clone_string() {
    println!("== Clone ==");

    let s1 = String::from("hello");
    let s2 = s1.clone(); // creates a deep copy of the heap data

    println!("s1 = {s1}, s2 = {s2}");

    println!();
}

fn ownership_and_functions() {
    println!("== Ownership and Functions ==");

    let s = String::from("hello"); // `s` comes into scope

    takes_ownership(s); // `s`'s value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // `x` comes into scope

    makes_copy(x); // Because `i32` implements the `Copy` trait,
                   // `x` does NOT move into the function,
                   // so it's okay to use `x` afterward.
    println!();
} // Here, `x` goes out of scope, then `s`. Because `s`'s value was moved,
  // nothing needs to be dropped for `s`.

fn takes_ownership(some_string: String) {
    // `some_string` comes into scope
    println!("{some_string}");
} // Here, `some_string` goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // `some_integer` comes into scope
    println!("{some_integer}");
} // Here, `some_integer` goes out of scope. Nothing special happens.

fn return_values_and_scope() {
    let _s1 = gives_ownership(); // `gives_ownership` moves its return
                                 // value into `_s1`

    let s2 = String::from("hello"); // `s2` comes into scope

    let _s3 = takes_and_gives_back(s2); // `s2` is moved into
                                        // `takes_and_gives_back`, which moves
                                        // its return value into `_s3`
} // Here, `_s3` goes out of scope and is dropped. `s2` was moved, so nothing
  // needs to be dropped for `s2`. `_s1` goes out of scope and is dropped.

fn gives_ownership() -> String {
    // `gives_ownership` moves its return value
    // to the calling function.

    let some_string = String::from("yours"); // `some_string` comes into scope

    some_string // `some_string` is returned,
                // moving ownership to the calling function
}

// This function takes a `String` and returns a `String`.
fn takes_and_gives_back(a_string: String) -> String {
    // `a_string` comes into scope

    a_string // `a_string` is returned,
             // moving ownership to the calling function
}

fn return_multiple_values() {
    println!("== Return Multiple Values ==");

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); // returns both values in a tuple

    println!("The length of '{s2}' is {len}.");

    println!();
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // `.len()` returns the length of the String

    (s, length) // returns ownership of `s` together with its length
}
