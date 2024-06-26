// Struct - Used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

// Function the work with Person struct
impl Person {
    // Construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get Full Name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set Last Name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

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

    c2.0 = 0;
    c2.1 = 255;
    c2.2 = 0;
    println!("Color: {} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("Abdulkader", "");
    println!("Person {} {}", p.first_name, p.last_name);

    p.set_last_name("Safi");
    println!("Person full Name: {}", p.full_name());

    println!("Person tuple {:?}", p.to_tuple())
}
