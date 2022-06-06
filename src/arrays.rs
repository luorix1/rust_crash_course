// Import mem from "std"
use std::mem;

// Array = fixed list where elements are of a single data type
pub fn run() {
    // Must specify type and length
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Re-assign value
    numbers[2] = 20;

    // Use the debug type when printing out arrays
    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}