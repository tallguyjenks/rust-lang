/**
 * 
*/

use std::env;

// useful to grab flag from command line args

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Bryan";
    let status = "100%";

    println!("Command: {}", command);

    if command == "hello" {
        println!("hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is: {}", status);
    }
}