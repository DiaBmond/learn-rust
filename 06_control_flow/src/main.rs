fn main() {
    if_expression_function(3);
    multiple_conditions(9);
    if_in_statement(true);
}

fn if_expression_function(number: i16) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn multiple_conditions(number: i16) {
    // The first branch whose condition evaluates to true is executed.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_statement(condition: bool) {
    // `if` is an expression, so its resulting value can be used
    // as the initializer in a `let` statement.
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Error: all branches of an `if` expression must evaluate to the same type.
    // let number = if condition { 5 } else { "six" };
}
