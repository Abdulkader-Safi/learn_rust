// Primitive str = Immutable fixed-length string somewhere in memory.
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("hello ");

    println!("{}", hello);
    // Get length
    println!("Length: {}", hello.len());

    hello.push('W');
    hello.push_str("orld!");

    println!("{}", hello);
    // Get length
    println!("Length: {}", hello.len());

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World': {}", hello.contains("World!"));

    // Replace
    println!("Replace: {}", hello.replace("World", "Abdulkader"));
}
