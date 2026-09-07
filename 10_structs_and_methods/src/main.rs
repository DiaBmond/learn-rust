#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Struct
struct Color(i32, i32, i32);

// Unit-Like Struct
struct AlwaysEqual;

// == Rectangle Implementation ==

impl Rectangle {
    // Method: `self` refers to the instance before the `.`.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method that checks whether the width is nonzero.
    fn width(&self) -> bool {
        self.width > 0
    }

    // Method with another `Rectangle` parameter.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function: no `self` parameter.
    // `Self` refers to `Rectangle` inside `impl Rectangle`.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    user_struct_example();
    tuple_struct_example();
    unit_like_struct_example();

    rectangle_with_separate_values();
    rectangle_with_tuple();
    rectangle_with_struct();

    debug_output_example();

    methods_example();
    associated_function_example();
}

// == User Struct ==

fn user_struct_example() {
    println!("== User Struct ==");

    let mut user1 = build_user(
        "someusername123".to_string(),
        "someone@example.com".to_string(),
    );

    println!("user1 email: {}", user1.email);

    user1.email = String::from("anotheremail@example.com");

    println!("user1 updated email: {}", user1.email);

    let user2 = build_user_from_existing(user1, "new@example.com".to_string());

    println!(
        "user2 email: {}, username: {}, active: {}, sign in count: {}",
        user2.email, user2.username, user2.active, user2.sign_in_count
    );

    println!();
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn build_user_from_existing(user: User, email: String) -> User {
    User { email, ..user }
}

// == Tuple Struct ==

fn tuple_struct_example() {
    println!("== Tuple Struct ==");

    let black = Color(0, 0, 0);

    println!("Black color values: {}, {}, {}", black.0, black.1, black.2);

    println!();
}

// == Unit-Like Struct ==

fn unit_like_struct_example() {
    println!("== Unit-Like Struct ==");

    let _subject = AlwaysEqual;

    println!();
}

// == Rectangle: Separate Values ==

fn rectangle_with_separate_values() {
    println!("== Rectangle with Separate Values ==");

    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_from_values(width, height)
    );

    println!();
}

fn area_from_values(width: u32, height: u32) -> u32 {
    width * height
}

// == Rectangle: Tuple ==

fn rectangle_with_tuple() {
    println!("== Rectangle with Tuple ==");

    let rectangle = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_from_tuple(rectangle)
    );

    println!();
}

fn area_from_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// == Rectangle: Struct ==

fn rectangle_with_struct() {
    println!("== Rectangle with Struct ==");

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_from_rectangle(&rectangle)
    );

    println!();
}

fn area_from_rectangle(rectangle: &Rectangle) -> u32 {
    // Borrow the Rectangle because ownership is not needed.
    rectangle.width * rectangle.height
}

// == Debug Output ==

fn debug_output_example() {
    println!("== Debug Output ==");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");

    let scale = 2;

    let rect2 = Rectangle {
        width: dbg!(30 * scale), // Prints the expression and returns its value.
        height: 50,
    };

    dbg!(&rect2); // Borrows `rect2` so `dbg!` does not take ownership.

    println!();
}

// == Methods ==

fn methods_example() {
    println!("== Methods ==");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}.", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!();
}

// == Associated Function ==

fn associated_function_example() {
    println!("== Associated Function ==");

    // `square` has no `self`, so call it with `Rectangle::square`.
    let square = Rectangle::square(20);

    println!("Square is {square:?}");
    println!("Square area is {}", square.area());

    println!();
}
