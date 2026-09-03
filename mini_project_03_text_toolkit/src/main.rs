use std::io;

fn main() {
    println!("== Start Text Toolkit ==");

    let text = read_text();
    let text = text.trim();

    println!("Input is: {text}");
    println!("First word is: {}", find_first_word(text));
    println!("Last word is: {}", find_last_word(text));

    let (chars, spaces, words, length) = count_text_stats(text);

    println!("Characters: {chars}");
    println!("Spaces: {spaces}");
    println!("Words: {words}");
    println!("Length: {length}");
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

fn count_text_stats(text: &str) -> (u32, u32, u32, u32) {
    let mut chars = 0;
    let mut spaces = 0;
    let mut length = 0;

    let bytes = text.as_bytes();

    for (_i, &item) in bytes.iter().enumerate() {
        length += 1;

        if item == b' ' {
            spaces += 1;
            continue;
        }

        chars += 1;
    }

    // Assumes words are separated by a single space.
    let words = spaces + 1;

    (chars, spaces, words, length)
}
