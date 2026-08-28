fn main() {
    if_expression_function(3);
    multiple_conditions(9);
    if_in_statement(true);

    loop_expression_function();
    loop_in_statement();
    loop_label_function();
    while_loop_function();
    for_loop_function();
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

fn loop_expression_function() {
    let mut count = 0;
    loop {
        if count > 5 {
            println!("count is: {count}");
            break;
        }
        count += 1;
    }
}

fn loop_in_statement() {
    // `loop` is an expression, so its resulting value can be
    // used as the initializer in a `let` statement.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_label_function() {
    let mut count = 0;

    // A loop label can be used to break out of an outer loop.
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop_function() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop_function() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        // `.rev()` iterates over the range in reverse: 3, 2, 1
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
