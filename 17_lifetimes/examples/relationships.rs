// The returned reference is tied only to the lifetime of x.
// The lifetime of y is independent of the return value.
fn first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// Return an owned String by transferring ownership to the caller.
// No lifetime annotation is needed.
fn create_message() -> String {
    String::from("Created inside function")
}

fn main() {
    let string1 = String::from("hello");
    let result;

    {
        let string2 = String::from("temporary");

        result = first(&string1, &string2);
    } // string2 is dropped here.

    // This compiles because result only borrows from string1.
    // The lifetime of string2 does not constrain the returned reference.
    println!("First: {result}");

    let message = create_message();
    println!("Owned: {message}");
}

// This example does not compile (E0515):
//
// fn invalid<'a>() -> &'a str {
//     let result = String::from("hello");
//     result.as_str()
// }
//
// result is dropped when the function returns.
// The returned reference would point to invalid data.
// Lifetime annotations cannot extend the lifetime of owned values.
