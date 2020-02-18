/**
 * Loops are used to iterate until a condition is met
 * in Rust there is an infinite loop
*/
pub fn run() {

    println!("\n\nLOOPS:\n");
    let mut count = 0;

    // infinite loop
    loop {
        count +=1;
        println!("Number: {}", count);
        if count == 20{
            break;
        }
    }

    println!("\n");
    count = 0;
    // While loops (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 && count != 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 && count != 0{
            println!("Fizz");
        } else if count % 5 == 0 && count != 0{
            println!("Buzz");
        } else {
            println!("{}", count)
        }
        // increment
        count += 1;
    }

    println!("\n");
    // for range
    for x in 0..100 {
        if x % 15 == 0 && x != 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 && x != 0{
            println!("Fizz");
        } else if x % 5 == 0 && x != 0{
            println!("Buzz");
        } else {
            println!("{}", x)
        }
    }
}