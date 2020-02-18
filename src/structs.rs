/**
 * structs used to create custom data types
*/

// struct
//struct Color {
//    red: u8,
//    green: u8,
//    blue: u8
//}

// tuple struct
//struct Color(u8, u8, u8);

// person struct
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct a person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    //let mut c = Color{
    //    red: 255,
    //    green: 0,
    //    blue: 0
    //};
    //println!("Color: R:{} G:{} B:{}", c.red, c.green, c.blue);
    
    // tuple struct
    //let mut c = Color(255, 0, 0);
    //c.0 = 200;
    //println!("Color: {} {} {}", c.0, c.1, c.2);

    // person stuff
    let mut p = Person::new("John", "Doe");
    println!("Person: {} {}", p.first_name, p.last_name);
    println!("Person Full Name: {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person Full Name: {}", p.full_name());
    println!("Person Full Name: {:?}", p.to_tuple());

}