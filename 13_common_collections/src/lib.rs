pub fn learn_vector(number: Option<i32>) -> Option<i32> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_correct() {
        let number = 7;
        let result = learn_vector(Some(number));

        assert_eq!(result, Some(7));
    }

    #[test]
    fn it_fail() {
        let result = learn_vector(None);

        assert_eq!(result, None);
    }
}
