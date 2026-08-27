fn main() {
    println!("Hello, Function!");

    another_function();
    parameter_function(9);
    two_parameters_function(10, "Hi");

    statement_and_expression();

    let x = function_returns_five();
    println!("function_returns_five() returns: {x}");

    let x = function_adds_five(4);
    println!("function_adds_five(4) returns: {x}");
}

fn another_function() {
    println!("Hello, Another Function!");
}

fn parameter_function(x: i32) {
    println!("Parameter is: {x}");
}

fn two_parameters_function(value: i32, text: &str) {
    println!("Value is: {}, Text is: {}", value, text);
}

fn statement_and_expression() {
    // This is a statement, but `6` is an expression within the statement.
    let x = 6;
    println!("x is: {x}");

    // Error: expected expression, found `let` statement.
    // let x = (let y = 3);

    // Parentheses can group an expression.
    // A `let` statement cannot be used directly where an expression is expected.

    let x = {
        let y = 6; // statement
        y + 3 // tail expression
    };

    // A block `{}` can contain statements and optionally end with an expression.
    // The value of the tail expression becomes the value of the block.
    // Here, `y + 3` evaluates to `9`, so `x` becomes `9`.
    //
    // If we write `y + 3;` instead, the value is discarded,
    // and the block evaluates to `()`.

    println!("x is: {x}");
}

fn function_returns_five() -> i32 {
    5
}

fn function_adds_five(x: i32) -> i32 {
    x + 5
}
