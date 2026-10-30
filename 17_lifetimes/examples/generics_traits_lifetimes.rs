use std::fmt::Display;

// 'a is a generic lifetime parameter.
// T is a generic type parameter.
// T: Display is a trait bound requiring T to implement Display.
//
// The returned reference is tied to both x and y.
// The announcement parameter does not affect the output lifetime.
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("hello");
    let string2 = String::from("hello world");

    // T is inferred as &str, which implements Display.
    let result = longest_with_an_announcement(&string1, &string2, "Comparing strings");

    println!("Longest: {result}");

    // T can be any type that implements Display.
    // Here, T is inferred as i32.
    let result2 = longest_with_an_announcement(&string1, &string2, 42);

    println!("Longest again: {result2}");
}
