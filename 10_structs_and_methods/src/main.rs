struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = build_user(
        "someusername123".to_string(),
        "someone@example.com".to_string(),
    );

    println!("user1 email: {}", user1.email);

    user1.email = String::from("anotheremail@example.com");

    println!("user1 updated email: {}", user1.email);

    let user2 = build_user_from_existing(user1, "new@example.com".to_string());

    println!("user2 email: {}, username: {}", user2.email, user2.username);
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
