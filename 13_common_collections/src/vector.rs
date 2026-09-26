pub enum LearnVector {
    BasicVector,
    ReadingVector,
    OwnerVector,
    IteratingVector,
    EnumVector,
}

pub fn learn_vector(learn: LearnVector) -> Option<String> {
    match learn {
        LearnVector::BasicVector => {
            let number = 7;
            return basic_vector(Some(number));
        }
        LearnVector::ReadingVector => {
            return reading_vectors();
        }
        LearnVector::OwnerVector => {
            return owner_with_vector();
        }
        LearnVector::IteratingVector => {
            return iterating_over_vector();
        }
        LearnVector::EnumVector => {
            return enum_with_vector();
        }
    }
}

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

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn enum_with_vector() -> Option<String> {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut text = String::new();

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => text.push_str(&format!("Int:{value} ")),
            SpreadsheetCell::Float(value) => text.push_str(&format!("Float:{value} ")),
            SpreadsheetCell::Text(value) => text.push_str(&format!("Text:{value} ")),
        }
    }

    Some(text)
}

#[cfg(test)]
mod tests_vector {
    use super::*;

    #[test]
    fn learn_basic_vector() {
        let result = learn_vector(LearnVector::BasicVector);

        assert_eq!(result, Some(String::from("The first element is 7")));
    }

    #[test]
    fn learn_reading_vector() {
        let result = learn_vector(LearnVector::ReadingVector);

        assert_eq!(result, Some(String::from("There is no hundredth element.")));
    }

    #[test]
    fn learn_owner_vector() {
        let result = learn_vector(LearnVector::OwnerVector);

        assert_eq!(result, Some(String::from("Vector length is 6")));
    }

    #[test]
    fn learn_iterating_over_vector() {
        let result = learn_vector(LearnVector::IteratingVector);

        assert_eq!(
            result,
            Some(String::from(
                "Values in v1: [100, 32, 57] Values in v2: [150, 82, 107]"
            ))
        );
    }

    #[test]
    fn learn_enum_with_vector() {
        let result = learn_vector(LearnVector::EnumVector);

        assert_eq!(result, Some(String::from("Int:3 Text:blue Float:10.12 ")));
    }
}
