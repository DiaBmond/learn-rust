use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;

    let mut username = String::new();
    file.read_to_string(&mut username)?;

    Ok(username)
}

fn main() -> Result<(), io::Error> {
    let username = read_username_from_file()?;

    println!("Username: {username}");

    Ok(())
}
