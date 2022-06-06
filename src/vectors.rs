// Import mem from "std"
use std::mem;

// Array = fixed list where elements are of a single data type
pub fn run() {
    // Must specify type and length
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    
    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Use the debug type when printing out vectors
    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers vec: {:?}", numbers);
}