// Vectors - Resizable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("{:?}", numbers);

    // Re-Assign Value
    numbers[2] = 20;
    println!("change 3 to 20: {:?}", numbers);

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

    println!("{:?}", numbers);
}
