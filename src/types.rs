// Primitive types
// Int: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits it takes up in memory)
// Float: f32, f64
// Boolean: bool
// Character: char
// Tuple
// Array
pub fn run() {
    // Default type is i32
    let x = 1;

    // Default type is f64
    let y = 2.5;

    // Add explicit type
    let z:i64 = 100000000000;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;
    
    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    let a1 = 'a';
    let face = '\u{1F600}';
    
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}