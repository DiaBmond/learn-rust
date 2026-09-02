use std::io;
fn main() {
    println!("== Start text tookit ==");
    let text = read_text();
    println!("input is: {text}");
}

fn read_text() -> String {
    println!("Please enter a text:");

    let mut read = String::new();

    io::stdin()
        .read_line(&mut read)
        .expect("Failed to read line");

    read
}
