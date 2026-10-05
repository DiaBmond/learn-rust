use std::fs::File;

fn main() {
    // let file_with_unwrap = File::open("Cargo.toml").unwrap();
    // let file_with_unwrap = File::open("does_not_exist.txt").unwrap();

    // println!("unwrap: {file_with_unwrap:?}");

    // let file_with_expect =
    //     File::open("Cargo.toml").expect("Cargo.toml should exist in this project");
    let file_with_expect =
        File::open("file_with_unwrap").expect("file_with_unwrap should exist in this project");

    println!("expect: {file_with_expect:?}");
}
