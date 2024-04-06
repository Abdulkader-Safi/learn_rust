// Primitive types
/*
 * Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, (number of bits they take in the memory)
 *              u unsigned can not have a negative number
 * Floats: f32, f64
 * Boolean (boll)
 * Characters (chr)
 * Tuples
 * Arrays
 */

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default f64
    let y = 2.5;

    // Add explicit type
    let g: i64 = 345902780379425;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;
    let not_is_active: bool = false;

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    // Character
    let a1 = 'a';
    let face = '\u{1F600}';

    println!(
        "{:?}",
        (x, y, g, is_active, not_is_active, is_greater, a1, face)
    )
}
