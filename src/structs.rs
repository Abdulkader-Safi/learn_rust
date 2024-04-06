// Struct - Used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct Color2(u8, u8, u8);

pub fn run() {
    // Traditional Struct
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("Color: {} {} {}", c.red, c.blue, c.green);

    c.red = 200;
    println!("Color: {} {} {}", c.red, c.blue, c.green);

    // Tuple Struct
    let mut c2 = Color2(255, 0, 0);
    println!("Color: {} {} {}", c2.0, c2.1, c2.2);

    c2.1 = 200;
    println!("Color: {} {} {}", c2.0, c2.1, c2.2);
}
