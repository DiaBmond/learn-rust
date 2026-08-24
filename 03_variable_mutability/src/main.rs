fn main() {
    let mut x = 5; // Can change the value
    x += 1;
    println!("The value of x is: {x}. It is a mutable variable");
    let y = 7; // Cannot change the value
    println!("The value of y is: {y}. It is an immutable variable");

    // Constants
    const ONE_HOUR_IN_SECONDS: u32 = 60 * 60;
    println!("One hour has {ONE_HOUR_IN_SECONDS} seconds");

    // Shadowing
    let z = 8;
    let z = z + 1; // Creates a new binding named `z`
    {
        let z = z + 2; // Creates a new binding named `z`
        println!("The value of z inside the scope: {z}");
    }
    println!("The value of z outside the scope: {z}"); // Uses the outer `z`

    let z = "String!"; // Creates a new binding named `z` with a different type

    println!("The value of z is now: {z}");
}
