pub fn basic_vector(number: Option<i32>) -> Option<String> {
    // Basic vector
    let Some(number) = number else {
        return None;
    };

    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    v.push(number);

    let one: &i32 = &v[0];
    println!("The first element is {one}");

    let one: Option<&i32> = v.get(0);

    let result: Option<String>;

    match one {
        Some(one) => {
            result = Some(format!("The first element is {one}"));
        }
        None => {
            result = Some(String::from("There is no first element."));
        }
    }

    result
}

pub fn reading_vectors() -> Option<String> {
    let v = vec![1, 2, 3, 4, 5];

    // This would panic because index 100 doesn't exist.
    // let _does_not_exist = &v[100];

    // get() safely returns None instead.
    let _does_not_exist = v.get(100);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // get() safely returns None instead.
    let hundredth: Option<&i32> = v.get(99);

    let result: Option<String>;

    match hundredth {
        Some(hundredth) => {
            result = Some(format!("The third element is {hundredth}"));
        }
        None => {
            result = Some(String::from("There is no hundredth element."));
        }
    }

    result
}

pub fn owner_with_vector() -> Option<String> {
    let mut v = vec![1, 2, 3, 4, 5];

    let _first = &v[0];

    v.push(6);

    // println!("The first element is {_first}");
    // Does not compile because `first` is still borrowed
    // when `v.push(6)` needs a mutable borrow.

    Some(format!("Vector length is {}", v.len()))
}

pub fn iterating_over_vector() -> Option<String> {
    let v1 = vec![100, 32, 57];
    for i in &v1 {
        println!("{i}");
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50;
    }
    Some(format!(
        "Values in v1: [{}, {}, {}] Values in v2: [{}, {}, {}]",
        v1[0], v1[1], v1[2], v2[0], v2[1], v2[2]
    ))
}
