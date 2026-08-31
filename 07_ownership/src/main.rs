fn main() {
    println!("ownership!");
    variable_scope();
    string_type();
    copy_vs_move();
    scope_and_assignment();
    clone_string();
    ownership_and_functions();
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
