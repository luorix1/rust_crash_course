// Reference pointer = Point to a resource in memory
pub fn run() {
    // Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("Values: {:?}", (arr1, arr2));

    
    // Vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("Values: {:?}", (&vec1, &vec2));
    
    // With non-primitives, if you assign another variable to a piece of data, the first variable loses the value
    // To prevent this, use & to reference it instead
    let vec3 = vec1;
    println!("Values: {:?}", (&vec3));
}