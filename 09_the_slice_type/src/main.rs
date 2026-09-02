fn main() {
    first_word_index_example();
    string_slices();
    slice_borrowing_example();
    array_slices();
}

// Before using slices:
// Returning only an index can become out of sync with the String.
fn first_word_index_example() {
    println!("== First Word Without Slices ==");

    let mut s = String::from("hello world");

    let word = first_word_index(&s);

    println!("index: {word}");

    s.clear();

    // `word` is still 5 even though `s` is now empty.
    println!("index after clear: {word}");

    println!();
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn string_slices() {
    println!("== String Slices ==");

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello: {hello}");
    println!("world: {world}");

    println!();
}

// A slice keeps the result tied to the borrowed String data.
fn slice_borrowing_example() {
    println!("== Slice and Borrowing ==");

    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("first word: {word}");

    // This would fail because `word` is still borrowing from `s`.
    //
    // s.clear();
    // println!("first word: {word}");

    // The borrow is no longer used here, so this is allowed.
    s.clear();

    println!("s after clear: {s}");

    println!();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn array_slices() {
    println!("== Array Slices ==");

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    println!("slice: {slice:?}");

    assert_eq!(slice, &[2, 3]);

    println!();
}
