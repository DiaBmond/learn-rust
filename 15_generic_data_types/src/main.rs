use std::cmp::PartialOrd;

// Generic Struct: x and y must have the same concrete type.
struct Point<T> {
    x: T,
    y: T,
}

// Generic Methods: available for Point<T> regardless of T.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// Concrete Type Implementation: only Point<f32> has this method.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Generic Function: works with any T that implements PartialOrd.
// Try removing `: PartialOrd` to see why the `>` comparison needs a trait bound.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    // Assumes the slice is not empty; otherwise, indexing will panic.
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // 1. Generic Function Definitions
    // The same function works with different concrete types.

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    // 2. Generic Struct Definitions
    // Point<T> requires x and y to have the same type.

    let integer = Point { x: 5, y: 10 };

    let float = Point {
        x: 1.0_f32,
        y: 4.0_f32,
    };

    // Uncomment to see a compile error:
    // x is i32, but y is a floating-point number.
    // To support different types, use Point<X, Y> instead.
    // let wont_work = Point { x: 5, y: 4.0 };

    // 3. Generic Method Definitions
    // x() and y() are available for both Point<i32> and Point<f32>.

    println!("integer.x = {} integer.y = {}", integer.x(), integer.y());
    println!("float.x = {} float.y = {}", float.x(), float.y());

    // distance_from_origin() is available only for Point<f32>.
    println!(
        "float.distance_from_origin = {}",
        float.distance_from_origin()
    );

    // Uncomment to see a compile error:
    // Point<i32> does not have the distance_from_origin() method.
    // println!("{}", integer.distance_from_origin());
}
