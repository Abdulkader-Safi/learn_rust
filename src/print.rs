pub fn run() {
    println!("Hello, world!");

    // Basic formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Abdulkader Safi", "Lebanon");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Abdulkader Safi", "Lebanon", "Code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Abdulkader Safi",
        activity = "BasketBall"
    );

    // Placeholders traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
