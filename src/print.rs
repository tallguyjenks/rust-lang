pub fn run(){
    // Print To Console
    println!("Hello from print.rs!");


    println!("Number: {}",1);

    println!("So this is basically a {} {}, well thats cool.", "template", "literal");

    println!("My name is {0} and i live on {1}, {0} likes to {2}.", "Bryan", "Earth", "code");
    println!("My name is {name}, and i like to {activity}", name="Bryan", activity="code");
    println!("Binary: {:b}, hex: {:x}, Octal: {:o}.", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10+10);
}
