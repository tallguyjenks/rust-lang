/*
* Primitive string = immutable fixed length string somewhere in memory
* string = growable, heap-allocated data structure - use when you need to modify or own string data
*/
pub fn run(){
    let hello = "hello"; // this is immutable and fixed
    let mut hello2 = String::from("Hello"); // to make the potentially mutable string mutable you must add `mut`

    //get length
    println!("Length: {}", hello.len());
    println!("Length: {}", hello2.len());

    hello2.push(' '); // only pushes a single character, must be wrapped in single quotes
    hello2.push_str("TEST"); // pushes a string onto the existing string

    // capacity in bytes
    println!("Capacity: {}", hello2.capacity());
    //Check if empty
    println!("Is Empty: {}", hello2.is_empty());
    // contains (substring)
    println!("Contains 'TEST' {}", hello2.contains("TEST"));

    // replace
    println!("Replace: {}", hello2.replace("T", "\u{16a6}"));

    // loop through string by whitespace
    for word in hello2.split_whitespace(){
        println!("SPLIT: {}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}\n\n", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("\n{}, {}", hello, hello2);
}