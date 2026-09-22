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
