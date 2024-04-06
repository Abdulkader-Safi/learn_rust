// Functions - Used to store blocks of code for re-use

pub fn run() {
    greeting("Hello", "Abdulkader");

    // Bind function values to variables
    let sum = add(5, 5);
    println!("Sum: {}", sum);

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));
}

// Void function
fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

// return function
fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}
