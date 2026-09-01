fn main() {
    references_and_borrowing();
    mutable_references();
}

fn references_and_borrowing() {
    println!("== References and Borrowing ==");

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    println!();
}

fn calculate_length(s: &String) -> usize {
    // `&String` is an immutable reference, so the value cannot be changed.
    // s.push_str(", world"); // Error: cannot borrow as mutable

    s.len()
}

fn mutable_references() {
    println!("== Mutable References ==");

    let mut s = String::from("hello");

    change(&mut s);

    println!("{s}");

    println!();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
