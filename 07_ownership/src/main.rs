fn main() {
    println!("ownership!");
    variable_scope();
    string_type();
    copy_vs_move();
    scope_and_assignment();
    clone_string();
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
