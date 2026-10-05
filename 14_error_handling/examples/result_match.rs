use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(file) => file,

        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => {
                    panic!("Problem creating the file: {error:?}");
                }
            },

            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    println!("File opened successfully: {file:?}");
}
