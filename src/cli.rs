use std::env;

pub fn run() {
    // Grabs command line inputs after "cargo run"
    let args: Vec<String> = env::args().collect();

    // Copy the first command line input after the execution command
    let command = args[1].clone();
    let name = "Brad";
    let status = "100%";

    println!("Args: {:?}", args);
    // println!("Args: {:?}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command.");
    }
}