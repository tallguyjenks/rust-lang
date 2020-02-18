/**
 * Arrays are fixed lists where elements are the same data type
*/

use std::mem;

pub fn run(){
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // get a sing val
    println!("Singe Val: {}", numbers[0]);

    // make mutable and re-assign a value
    let mut numbers2: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers2);
    numbers2[2] = 20;
    println!("{:?}", numbers2);

    // get array length
    println!("Array Length: {}", numbers2.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers2));

    // get slice
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice);
}
