// Struct = Used to create custom data types

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct TupleColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

// Essentially, the constructor for "Person"
impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Print full name
    fn full_name(&self) -> String {
        // Works in the same way as with format in strings
        format!("{} {}", self.first_name, self.last_name)
    }

    // Change last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut d = TupleColor(255, 0, 0);
    d.1 = 200;
    println!("Color: {} {} {}", d.0, d.1, d.2);

    let mut p = Person::new("John", "Doe");
    println!("Person: {} {}", p.first_name, p.last_name);

    // Call full_name()
    println!("Person: {}", p.full_name());

    // Change last name
    p.set_last_name("Lennon");
    println!("Person: {}", p.full_name());

    // Return name as tuple
    println!("Person Tuple: {:?}", p.to_tuple());
}