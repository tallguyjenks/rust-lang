/*
Primitive Types~
    * Integers: 
        - u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory) unsigned/integers unsigned cant take negative values?
    * Floats: 
        - f32, f64
    * Boolean: 
        - Bool
    * Character
        - Char
    * Tuples (basically lists?)
    * Arrays
*/

pub fn run(){
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z:i64 = 45454545454545;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater = 10 < 5;

    // Char
    let a1 = 'a';
    let face = '\u{1f600}';
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}