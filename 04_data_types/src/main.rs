use std::io;

fn main() {
    /* Scalar Types */
    // Signed integer types: -(2^(n-1)) to (2^(n-1))-1
    println!("Signed 8-bit MIN:{}", i8::MIN); // -128
    println!("Signed 8-bit MAX:{}", i8::MAX); // 127

    println!("Signed 16-bit MIN:{}", i16::MIN); // -32768
    println!("Signed 16-bit MAX:{}", i16::MAX); // 32767

    println!("Signed 32-bit MIN:{}", i32::MIN); // -2147483648
    println!("Signed 32-bit MAX:{}", i32::MAX); // 2147483647

    println!("Signed 64-bit MIN:{}", i64::MIN); // -9223372036854775808
    println!("Signed 64-bit MAX:{}", i64::MAX); // 9223372036854775807

    println!("Signed 128-bit MIN:{}", i128::MIN); // -170141183460469231731687303715884105728
    println!("Signed 128-bit MAX:{}", i128::MAX); // 170141183460469231731687303715884105727

    // Unsigned integer types: 0 to (2^n)-1
    println!("Unsigned 8-bit MIN:{}", u8::MIN); // 0
    println!("Unsigned 8-bit MAX:{}", u8::MAX); // 255

    println!("Unsigned 16-bit MIN:{}", u16::MIN); // 0
    println!("Unsigned 16-bit MAX:{}", u16::MAX); // 65535

    println!("Unsigned 32-bit MIN:{}", u32::MIN); // 0
    println!("Unsigned 32-bit MAX:{}", u32::MAX); // 4294967295

    println!("Unsigned 64-bit MIN:{}", u64::MIN); // 0
    println!("Unsigned 64-bit MAX:{}", u64::MAX); // 18446744073709551615

    println!("Unsigned 128-bit MIN:{}", u128::MIN); // 0
    println!("Unsigned 128-bit MAX:{}", u128::MAX); // 340282366920938463463374607431768211455

    // Architecture-dependent integer types
    println!("Signed isize MIN:{}", isize::MIN); // -9223372036854775808 Architecture-dependent (64-bit on my machine)
    println!("Signed isize MAX:{}", isize::MAX); // 9223372036854775807 Architecture-dependent (64-bit on my machine)
    println!("Unsigned usize MIN:{}", usize::MIN); // 0
    println!("Unsigned usize MAX:{}", usize::MAX); // 18446744073709551615 Architecture-dependent (64-bit on my machine)

    // let x = 1000; // type inference
    // let x: u16 = 1000; // type annotation
    // let x = 1000u16; // type suffix

    // Integer Literals
    let x = 10_000; // `_` is a visual separator
    println!("{x}");

    let x = 0xff; // hexadecimal (base 16)
    println!("{x}");

    let x = 0o77; // octal (base 8)
    println!("{x}");

    let x = 0b1110_0001; // binary (base 2)
    println!("{x}");

    let x = b'A'; // byte literal, type `u8`
    println!("{x}");

    // Overflow
    // Compile-time overflow
    // let x: u8 = 255;
    // let y = x + 1;
    // println!("{y}");

    // Runtime overflow
    // Default dev profile: input 255 -> panics
    // Default release profile: input 255 -> wraps to 0
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let x: u8 = input.trim().parse().expect("Please type a number");

    let y = x + 1;

    println!("{y}");

    // Floating-Point Types
    println!("Floating-Point Types");
    let x = 2.0; // f64
    println!("{x}");

    let x: f32 = 3.0; // f32
    println!("{x}");

    // Numeric Operations
    println!("Numeric Operations");

    // addition
    let x = 5 + 10;
    println!("{x}");

    // subtraction
    let x = 95.5 - 4.3;
    println!("{x}");

    // multiplication
    let x = 4 * 30;
    println!("{x}");

    // division
    let x = 56.7 / 32.2;
    println!("{x}");

    let x = -5 / 3; // integer division truncates toward zero
    println!("{x}");

    // remainder
    let x = 43 % 5;
    println!("{x}");

    // The Boolean Type
    println!("The Boolean Type");
    let x = true;
    println!("{x}");

    let x: bool = false; // explicit type annotation
    println!("{x}");

    // The Character Type
    println!("The Character Type");
    let x: char = 'ℤ'; // explicit type annotation
    println!("{x}");

    let x = '😻';
    println!("{x}");

    /* Compound Types */
    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {x}, y: {y}, z: {z}");
    let five_hundred = tup.0;
    println!("tup.0: {five_hundred}");
    let six_point_four = tup.1;
    println!("tup.1: {six_point_four}");
    let one = tup.2;
    println!("tup.2: {one}");

    // The Array Type
    let arr = [1, 2, 3, 4, 5];
    println!("arr[0]: {}", arr[0]);
    println!("arr[1]: {}", arr[1]);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("First month: {}", months[0]);

    // Explicit type and length
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let a = [3; 5]; // [value; length]
    println!("{:?}", a); // [3, 3, 3, 3, 3]

    println!("Please enter an array index.");

    let mut index = String::new();

    ////////////////////////////////////////////
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // Invalid array element access can panic at runtime
    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");
}
