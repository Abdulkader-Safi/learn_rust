// loops - Used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        print!("Number: {} ", count);

        if count == 20 {
            break;
        }
    }

    println!();

    count = 1;
    // While loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("{} - FizzBuzz", count);
        } else if count % 3 == 0 {
            println!("{} - Fizz", count);
        } else if count % 5 == 0 {
            println!("{} - Buzz", count);
        } else {
            println!("{} - nothing", count);
        }
        count += 1;
    }

    // For Range (FizzBuzz)
    for x in 0..100 {
        if x % 15 == 0 {
            println!("{} - FizzBuzz", x);
        } else if x % 3 == 0 {
            println!("{} - Fizz", x);
        } else if x % 5 == 0 {
            println!("{} - Buzz", x);
        } else {
            println!("{} - nothing", x);
        }
    }
}
