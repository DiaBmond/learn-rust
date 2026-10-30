// Lifetime Elision Rule 1:
// Each input reference is assigned its own lifetime.
//
// Lifetime Elision Rule 2:
// If there is exactly one input lifetime,
// the output reference uses that same lifetime.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// This function has two parameters but only one input reference.
// Rule 2 applies, so the returned reference is tied to s.
fn identity(s: &str, _number: usize) -> &str {
    s
}

fn main() {
    let sentence = String::from("Hello Rust world");

    let word = first_word(&sentence);
    println!("First word: {word}");

    let text = identity(&sentence, 42);
    println!("Identity: {text}");
}

// This example does not compile (E0106):
//
// fn choose(x: &str, y: &str) -> &str {
//     x
// }
//
// Rule 1 assigns separate lifetimes to x and y.
// Rule 2 does not apply because there are two input lifetimes.
// Rule 3 does not apply because this function has no self parameter.
//
// The compiler cannot infer the output lifetime.
// An explicit lifetime annotation is required.
