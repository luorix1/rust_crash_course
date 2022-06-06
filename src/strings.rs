// Primitive string = immutable, fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - use when you need to modify or own string data
pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Push (push only allows char)
    hello.push('W');

    // Use push_str instead!
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World': {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    // If assertion passes, nothing shows up
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello);
}