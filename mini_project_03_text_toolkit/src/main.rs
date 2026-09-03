use std::io;

fn main() {
    println!("== Start Text Toolkit ==");
    println!("Please enter text:");
    let mut text = read_text();

    loop {
        println!("Input is: {text}");
        println!("First word is: {}", find_first_word(&text));
        println!("Last word is: {}", find_last_word(&text));

        let (chars, spaces, words, length) = count_text_stats(&text);

        println!("Characters: {chars}");
        println!("Spaces: {spaces}");
        println!("Words: {words}");
        println!("Length: {length}");

        println!();
        let state: u8 = loop {
            println!("== Choose Next Step ==");
            println!("Enter 1 to edit text");
            println!("Enter 0 to exit");
            println!("Your choice:");

            let choice = read_text();

            let choice: u8 = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if choice == 0 || choice == 1 {
                break choice;
            }
        };

        if state == 1 {
            edit_mode(&mut text);
            continue;
        } else {
            println!("*** Exit");
            break;
        }
    }
}

fn read_text() -> String {
    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    text.trim().to_string()
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

fn edit_mode(text: &mut String) {
    println!("*** Edit Mode");

    let state = loop {
        println!("== Choose Edit Tool ==");
        println!("Enter 1 to add a word");
        println!("Enter 2 to delete the last word");
        println!("Your choice:");

        let choice = read_text();

        let choice: u8 = match choice.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 1 || choice == 2 {
            break choice;
        }

        println!("Please choose again.");
    };

    if state == 1 {
        println!("Enter a new word:");

        let new_word = read_text();

        text.push_str(" ");
        text.push_str(&new_word);
        println!("**************");
        println!();
    } else if state == 2 {
        let bytes = text.as_bytes();
        let mut position = 0;

        for (i, &item) in bytes.iter().enumerate().rev() {
            if item == b' ' {
                position = i;
                break;
            }
        }

        let new_text = String::from(&text[..position]);

        text.clear();
        text.push_str(&new_text);
        println!("**************");
        println!();
    }
}
