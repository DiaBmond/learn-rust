use std::io;

fn main() {
    println!("Number Toolkit");
    let number = read_positive_integer();
    println!("{number}");
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
