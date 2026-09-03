use std::io;

fn main() {
    println!("== Start Text Toolkit ==");

    let mut text = read_text();

    loop {
        let trimmed = text.trim();

        println!("Input is: {trimmed}");
        println!("First word is: {}", find_first_word(trimmed));
        println!("Last word is: {}", find_last_word(trimmed));

        let (chars, spaces, words, length) = count_text_stats(trimmed);

        println!("Characters: {chars}");
        println!("Spaces: {spaces}");
        println!("Words: {words}");
        println!("Length: {length}");

        let state: u8 = loop {
            println!("== Choose Next Step ==");
            println!("Enter 1 to edit text");
            println!("Enter 0 to exit");
            println!("Your choice:");

            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            let choice: u8 = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if choice == 0 || choice == 1 {
                break choice;
            }
        };

        if state == 1 {
            println!("*** Edit text");
            continue;
        } else {
            println!("*** Exit");
            break;
        }
    }
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
