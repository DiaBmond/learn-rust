use std::thread;

pub(crate) fn closure_type_inference_demo() {
    // Explicit type annotations
    let add_one_v2 = |x: u32| -> u32 { x + 1 };

    // Inferred parameter and return types with a block body
    let add_one_v3 = |x| x + 1;

    // Single-expression closure without braces
    let add_one_v4 = |x| x + 1;

    println!("v2: {}", add_one_v2(5_u32));
    println!("v3: {}", add_one_v3(5_u32));
    println!("v4: {}", add_one_v4(5_u32));

    // The argument type is inferred as String.
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    println!("String: {s}");

    // Uncomment this line to observe error E0308.
    // let n = example_closure(5);
}

pub(crate) fn immutable_capture_demo() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

pub(crate) fn mutable_capture_demo() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    // Uncomment to observe the borrow checker error.
    // println!("Before calling closure: {list:?}");

    borrows_mutably();
    println!("After calling closure: {list:?}");
}

pub(crate) fn move_capture_demo() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // Uncomment to observe the ownership error.
    // println!("After moving: {list:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closure_add_one_variants() {
        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        let add_one_v3 = |x| x + 1;
        let add_one_v4 = |x| x + 1;

        assert_eq!(add_one_v2(5_u32), 6);
        assert_eq!(add_one_v3(5_u32), 6);
        assert_eq!(add_one_v4(5_u32), 6);
    }

    #[test]
    fn closure_infers_string_type() {
        let example_closure = |x| x;

        let result = example_closure(String::from("hello"));

        assert_eq!(result, "hello");
    }

    #[test]
    fn closure_captures_immutable_reference() {
        let list = vec![1, 2, 3];

        let only_borrows = || list.len();

        assert_eq!(only_borrows(), 3);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn closure_captures_mutable_reference() {
        let mut list = vec![1, 2, 3];

        let mut borrows_mutably = || list.push(7);

        borrows_mutably();

        assert_eq!(list, vec![1, 2, 3, 7]);
    }

    #[test]
    fn closure_moves_ownership_into_thread() {
        let list = vec![1, 2, 3];

        let handle = thread::spawn(move || list.len());

        let result = handle.join().unwrap();

        assert_eq!(result, 3);
    }

    #[test]
    fn closure_fn_can_be_called_multiple_times() {
        let value = String::from("Rust");

        let get_length = || value.len();

        assert_eq!(get_length(), 4);
        assert_eq!(get_length(), 4);
    }

    #[test]
    fn closure_fn_mut_updates_captured_value() {
        let mut count = 0;

        let mut increment = || {
            count += 1;
            count
        };

        assert_eq!(increment(), 1);
        assert_eq!(increment(), 2);
    }

    #[test]
    fn closure_fn_once_moves_captured_value() {
        let message = String::from("Hello");

        let consume = || message;

        assert_eq!(consume(), "Hello");

        // Uncomment to observe a compile error.
        // consume();
    }
}
