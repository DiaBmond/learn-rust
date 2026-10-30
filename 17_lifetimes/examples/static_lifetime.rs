// String literals have the 'static lifetime.
// Their data remains valid for the entire duration of the program.
fn greeting() -> &'static str {
    "Hello Rust"
}

fn main() {
    let message: &'static str = greeting();

    println!("{message}");

    {
        let temporary_reference: &'static str = "I am static";

        println!("{temporary_reference}");
    }

    // temporary_reference goes out of scope here.
    // However, the string literal's data remains valid.
    // A reference variable's scope is not the same as
    // the lifetime of the data it references.
}

// This example does not compile (E0515):
//
// fn invalid() -> &'static str {
//     let s = String::from("hello");
//     s.as_str()
// }
//
// s is dropped when the function returns.
// The returned reference would point to invalid data.
// Adding 'static does not extend the lifetime of a local String.
