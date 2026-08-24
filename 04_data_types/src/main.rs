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
}
