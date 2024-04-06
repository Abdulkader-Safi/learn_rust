// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    println!("Hello, world!");
    let name = "Abdulkader";

    let mut age = 24;
    println!("My name is {} and I am {}", name, age);
    age = 30;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple var
    let (my_name, my_age) = ("Abdulkader", 24);
    println!("{} is {}", my_name, my_age);
}
