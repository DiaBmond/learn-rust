// == Basic Enum ==
enum IpAddrKind {
    V4,
    V6,
}

// == Enum with Data ==
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// == Enum Variants with Different Data ==
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// == Message Implementation ==
impl Message {
    // Method: `self` refers to the enum value before the `.`.
    fn call(&self) {
        println!("Message called");
    }
}

fn main() {
    basic_enum_example();
    enum_with_data_example();
    message_enum_example();
}

// == Basic Enum ==
fn basic_enum_example() {
    println!("== Basic Enum ==");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Both variants are values of the same type: `IpAddrKind`.
    route(four);
    route(six);

    println!();
}

fn route(_ip_kind: IpAddrKind) {
    println!("Routing IP address");
}

// == Enum with Data ==
fn enum_with_data_example() {
    println!("== Enum with Data ==");

    // Each variant can store different types and amounts of data.
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    println!();
}

// == Message Enum ==
fn message_enum_example() {
    println!("== Message Enum ==");

    // Variant with no associated data.
    let quit = Message::Quit;

    // Variant with named fields, similar to a struct.
    let _move_message = Message::Move { x: 10, y: 20 };

    // Variant with one `String`, similar to a tuple struct.
    let write_message = Message::Write(String::from("hello"));

    // Variant with three `i32` values, similar to a tuple struct.
    let _change_color = Message::ChangeColor(255, 0, 0);

    // Enum values can use methods defined in an `impl` block.
    quit.call();
    write_message.call();

    println!();
}
