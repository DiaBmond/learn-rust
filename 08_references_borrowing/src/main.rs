fn main() {
    references_and_borrowing();
}

fn references_and_borrowing() {
    println!("== References and Borrowing ==");
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    println!("");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
