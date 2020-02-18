/**
 * Will probably use these more than arrays
 * Vectors are re-sizable arrays
*/
use std::mem;
pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // add on to vector
    numbers.push(5);
    numbers.push(6);
    numbers.pop();

    println!("\n\n\nVectors now\n{:?}", numbers);

    // get a sing val
    println!("Single Val: {}", numbers[0]);

    // get array length
    println!("Vector Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}