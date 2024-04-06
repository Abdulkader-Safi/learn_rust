// Vectors - Resizable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("{:?}", numbers);

    // Re-Assign Value
    numbers[2] = 20;
    println!("change 3 to 20: {:?}", numbers);
    numbers[2] = 3;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);
    println!("add 5, 6: {:?}", numbers);

    // Pop off last value
    numbers.pop();
    println!("pop last element: {:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // get vector length
    println!("vector length: {}", numbers.len());

    // vectors are stack alloted
    println!("vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers);
}
