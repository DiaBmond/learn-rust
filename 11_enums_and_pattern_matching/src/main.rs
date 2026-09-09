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
        match self {
            Message::Quit => {
                println!("Quit");
            }
            Message::Move { x, y } => {
                println!("Move to x: {x}, y: {y}");
            }
            Message::Write(text) => {
                println!("Write: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change color to RGB({r}, {g}, {b})");
            }
        }
    }
}

fn main() {
    basic_enum_example();
    enum_with_data_example();
    message_enum_example();

    option_example();

    match_example();
    catch_all_example();

    if_let_example();
    let_else_example();
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
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    print_ip(home);
    print_ip(loopback);

    println!();
}

fn print_ip(ip: IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => {
            println!("IPv4: {a}.{b}.{c}.{d}");
        }
        IpAddr::V6(address) => {
            println!("IPv6: {address}");
        }
    }
}

// == Message Enum ==
fn message_enum_example() {
    println!("== Message Enum ==");

    let quit = Message::Quit;
    let move_message = Message::Move { x: 10, y: 20 };
    let write_message = Message::Write(String::from("hello"));
    let change_color = Message::ChangeColor(255, 0, 0);

    quit.call();
    move_message.call();
    write_message.call();
    change_color.call();

    println!();
}

// == Option Enum ==
fn option_example() {
    println!("== Option Enum ==");

    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    println!("Some(5) + 1 = {:?}", plus_one(some_number));
    println!("None + 1 = {:?}", plus_one(absent_number));

    println!();
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

// == Match ==
fn match_example() {
    println!("== Match ==");

    let number = 2;

    // `match` is an expression, so its result can be assigned to a variable.
    let text = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };

    println!("{number} is {text}");

    println!();
}

// == Catch-All Patterns ==
fn catch_all_example() {
    println!("== Catch-All Patterns ==");

    let dice_roll = 9;

    // A named catch-all pattern binds the matched value.
    match dice_roll {
        3 => println!("Add fancy hat"),
        7 => println!("Remove fancy hat"),
        other => println!("Move {other} spaces"),
    }

    let dice_roll = 5;

    // `_` matches any remaining value without binding it.
    match dice_roll {
        3 => println!("Add fancy hat"),
        7 => println!("Remove fancy hat"),
        _ => (),
    }

    println!();
}

// == If Let ==
fn if_let_example() {
    println!("== If Let ==");

    let config_max = Some(3u8);

    // `if let` handles one pattern without writing a full `match`.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    println!();
}

// == Let Else ==
fn let_else_example() {
    println!("== Let Else ==");

    let ip = IpAddr::V6(String::from("::1"));

    // Continue only when `ip` matches the `V6` variant.
    let IpAddr::V6(address) = ip else {
        println!("Not an IPv6 address");
        return;
    };

    println!("IPv6 address: {address}");

    println!();
}
