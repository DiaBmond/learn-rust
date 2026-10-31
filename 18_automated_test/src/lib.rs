pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(value: u64) -> u64 {
    value + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1. Basic assertions

    #[test]
    fn addition_works() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn add_two_returns_expected_value() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn add_two_changes_original_value() {
        let original = 10;
        let result = add_two(original);

        assert_ne!(result, original);
    }

    // 2. Boolean assertions with Rectangle

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn equal_rectangles_cannot_hold_each_other() {
        let first = Rectangle {
            width: 5,
            height: 5,
        };
        let second = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(!first.can_hold(&second));
    }

    // 3. Equality with custom types

    #[test]
    fn points_are_equal() {
        let first = Point { x: 10, y: 20 };
        let second = Point { x: 10, y: 20 };

        assert_eq!(first, second);
    }

    #[test]
    fn points_are_not_equal() {
        let first = Point { x: 10, y: 20 };
        let second = Point { x: 10, y: 30 };

        assert_ne!(first, second);
    }

    // 4. Custom failure messages

    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = greeting(name);

        assert!(
            result.contains(name),
            "Expected greeting to contain `{name}`, but got `{result}`"
        );
    }

    // 5. Testing expected panics

    #[test]
    #[should_panic]
    fn guess_above_100_panics() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn guess_above_100_panics_with_correct_message() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn guess_below_1_panics_with_correct_message() {
        Guess::new(0);
    }

    #[test]
    fn guess_accepts_boundary_values() {
        let minimum = Guess::new(1);
        let maximum = Guess::new(100);

        assert_eq!(minimum.value(), 1);
        assert_eq!(maximum.value(), 100);
    }

    // 6. Tests returning Result<T, E>

    #[test]
    fn addition_returns_ok() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(format!("Expected 4, but got {result}"))
        }
    }

    #[test]
    fn valid_number_parses_successfully() -> Result<(), std::num::ParseIntError> {
        let number: i32 = "42".parse()?;

        assert_eq!(number, 42);

        Ok(())
    }

    #[test]
    fn invalid_number_returns_error() {
        let result = "hello".parse::<i32>();

        assert!(
            result.is_err(),
            "Expected parsing an invalid number to return Err"
        );
    }

    // 7. Intentional failures for experimentation

    #[test]
    #[ignore = "Practice reading assert_eq! failure output"]
    fn demo_assert_eq_failure() {
        assert_eq!(add_two(2), 5);
    }

    #[test]
    #[ignore = "Practice reading assert! failure output"]
    fn demo_assert_failure() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!larger.can_hold(&smaller));
    }

    #[test]
    #[ignore = "Practice reading custom failure messages"]
    fn demo_custom_message_failure() {
        let result = greeting("Carol");

        assert!(
            result.contains("Carosl"),
            "Greeting did not contain expected name, value was `{result}`"
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    #[ignore = "Practice reading unexpected panic messages"]
    fn demo_wrong_panic_message() {
        Guess::new(0);
    }

    #[test]
    #[should_panic]
    #[ignore = "Practice a test that fails because no panic occurs"]
    fn demo_missing_panic() {
        Guess::new(50);
    }

    #[test]
    #[ignore = "Practice returning Err from a test"]
    fn demo_result_error() -> Result<(), String> {
        Err(String::from("Intentional test error"))
    }

    #[test]
    #[ignore = "Practice propagating an error with the ? operator"]
    fn demo_question_mark_error() -> Result<(), std::num::ParseIntError> {
        let _: i32 = "hello".parse()?;

        Ok(())
    }
}
