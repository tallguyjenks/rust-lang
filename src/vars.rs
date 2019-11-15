pub fn run(){
    /* 
    * Variables hold primitive data or references to data
    * variables are immutable by default
    * Rust is a block-scoped language
    */

    let name = "Bryan";
    let mut age = 55;
    println!("My name is {} and i am {} years old", name, age);
    age = 56;
    println!("My name is {} and i am {} years old", name, age);

    // Define Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let ( my_name, my_age ) = ("bryan", 55);
    println!("{} is {}",my_name,my_age);
}