use std::fs::File;

fn main() {
    // let result = File::open("Cargo.toml");
    let result = File::open("does_not_exist.txt");

    let Ok(file) = result else {
        println!("Could not open the file");
        return;
    };

    println!("File opened successfully: {file:?}");
}
