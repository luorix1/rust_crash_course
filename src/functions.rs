// Function = store blocks of re-usable code
pub fn run() {
    greeting("Hello", "Jane");

    // Bind return value to variable
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 10;
    // An inline version of function writing
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));
}

// Function with no return value
fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

// Explicit typing for return value
fn add(n1: i32, n2: i32) -> i32 {
    // No semi-colon after the value you want to return!
    n1 + n2
}