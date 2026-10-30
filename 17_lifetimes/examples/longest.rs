// The returned reference is tied to both input references.
// It can only be used while both inputs are still valid.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);

        // Both borrowed values are still valid here.
        println!("The longest string is {result}");
    }

    // This example would fail with E0597:
    //
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(&string1, &string2);
    // }
    // println!("{result}");
    //
    // The returned reference is constrained by both inputs.
    // Using it after string2 is dropped violates that contract.
}
