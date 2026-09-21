pub fn basic_vector(number: Option<i32>) -> Option<i32> {
    //Basic vector
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

    match one {
        Some(one) => println!("The first element is {one}"),
        None => println!("There is no first element."),
    }

    one.copied()
}

pub fn reading_vectors() {
    let v = vec![1, 2, 3, 4, 5];

    let _does_not_exist = &v[100];
    let _does_not_exist = v.get(100);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
