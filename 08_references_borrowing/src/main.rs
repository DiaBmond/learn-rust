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

    // Multiple mutable references to the same value are not allowed
    // at the same time.
    //
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    //
    // Error[E0499]: cannot borrow `s` as mutable more than once at a time
    // println!("{r1}, {r2}");

    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // `_r1` goes out of scope, so `s` can be mutably borrowed again.

    let r2 = &mut s;
    println!("r2: {r2}");

    // An immutable reference and a mutable reference cannot be used
    // at the same time.
    //
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;
    //
    // Error[E0502]: cannot borrow `s` as mutable because it is also
    // borrowed as immutable
    // println!("{r1}, {r2}, and {r3}");

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{r1} and {r2}");
    // `r1` and `r2` are no longer used after this point,
    // so the mutable reference below is allowed.

    let r3 = &mut s;
    println!("{r3}");

    println!();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
