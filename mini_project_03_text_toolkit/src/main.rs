use std::io;

fn main() {
    println!("== Start Text Toolkit ==");

    let text = read_text();

    println!("Input is: {}", text.trim());
    println!("First word is: {}", find_first_word(text.trim()));
    println!("Last word is: {}", find_last_word(text.trim()));
}

fn read_text() -> String {
    println!("Please enter text:");

    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    text
}

fn find_first_word(text: &str) -> &str {
    let bytes = text.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[..i];
        }
    }

    &text[..]
}

fn find_last_word(text: &str) -> &str {
    let bytes = text.as_bytes();

    for (i, &item) in bytes.iter().enumerate().rev() {
        if item == b' ' {
            return &text[i + 1..];
        }
    }

    &text[..]
}
