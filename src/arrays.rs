// Arrays - Fixed list where elements are the same data type

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Does not work because it expects array of 5 element and i am giving it 4 only
    // let numbers2: [i32; 5] = [1, 2, 3, 4];
    // println!("{:?}", numbers2);

    // Re-Assign Value
    numbers[2] = 20;

    // Get single val
    println!("Single value: {}", numbers[0]);

    // get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack alloted
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);

    println!("{:?}", numbers);
}
