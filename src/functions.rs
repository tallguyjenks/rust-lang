/**
 * functions store blocks of code for re-use
*/
pub fn run() {
    greeting("Hello", "Jane");
    let get_sum = add(5, 5);
    println!("{}", get_sum);

    let n3: i32 = 6;
    //closure
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; // n3 is an outside variable but can be used with a closure
    println!("C Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}


fn add (n1: i32, n2: i32) -> i32 { // returning an i32 
    n1 + n2
}