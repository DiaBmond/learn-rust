struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Structs
struct Color(i32, i32, i32);

// Unit-like struct
struct AlwaysEqual;

fn main() {
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

    // Tuple Structs
    let black = Color(0, 0, 0);
    println!("Black color values: {}, {}, {}", black.0, black.1, black.2);

    let _subject = AlwaysEqual;

    example_program_v1();
    example_program_v2();
    example_program_v3();
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

fn example_program_v1() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_v1(width1, height1)
    );
}

fn area_v1(width: u32, height: u32) -> u32 {
    width * height
}

fn example_program_v2() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_v2(rect1)
    );
}

fn area_v2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn example_program_v3() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_v3(&rect1)
    );
}

fn area_v3(rectangle: &Rectangle) -> u32 {
    // Borrow the Rectangle because ownership is not needed.
    rectangle.width * rectangle.height
}
