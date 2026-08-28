use std::io;

fn main() {
    println!("Number Toolkit");
    let number = read_positive_integer();
    println!("Number: {number}");
    println!("Even: {}", is_even(number));
    println!("Prime: {}", is_prime(number));
    println!("Divisible by 3: {}", is_divisible_3(number));
    println!("Divisible by 5: {}", is_divisible_5(number));
}

fn read_positive_integer() -> u32 {
    loop {
        println!("Please enter a positive integer:");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if number > 0 {
            return number;
        }
    }
}

fn is_even(number: u32) -> bool {
    number % 2 == 0
}

fn is_prime(number: u32) -> bool {
    if number < 2 {
        return false;
    }

    for index in 2..number {
        if number % index == 0 {
            return false;
        }
    }
    true
}

fn is_divisible_3(number: u32) -> bool {
    number % 3 == 0
}

fn is_divisible_5(number: u32) -> bool {
    number % 5 == 0
}
