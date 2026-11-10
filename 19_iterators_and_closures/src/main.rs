#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn closure_type_inference_demo() {
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

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
    closure_type_inference_demo();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn giveaway_with_preference() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_preference = Some(ShirtColor::Red);

        assert_eq!(store.giveaway(user_preference), ShirtColor::Red);
    }

    #[test]
    fn giveaway_without_preference() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_preference = None;

        assert_eq!(store.giveaway(user_preference), ShirtColor::Blue);
    }

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
}
