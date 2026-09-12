mod text {
    mod analyze {
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
    }

    mod edit {
        use std::io;

        fn read_text() -> String {
            let mut text = String::new();

            io::stdin()
                .read_line(&mut text)
                .expect("Failed to read line");

            text.trim().to_string()
        }

        fn add_word(text: &mut String) {
            println!("Enter a new word:");

            let new_word = read_text();

            text.push_str(" ");
            text.push_str(&new_word);
            println!("**************");
            println!();
        }
    }
}
