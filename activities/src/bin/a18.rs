// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

//First Try

// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * Implement Debug print functionality using `derive`
// #[Derive(Debug)]
// struct Adult {
// //   * The structure must contain the person's name and age
//     name: String,
//     age: u8, //up to 255 (u8)
// }

// // * Implement a `new` function for the `Adult` structure that returns a Result:
// fn check_age(age:) -> Result<(), u8> {
//     if 
// }
//     if
// }
// fn main() {
//     let age = check_age(25);

// }
// not a chance

#[derive(Debug)]
struct Adult {
    age: u8,
    name: String,
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> { //make a new adult
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string()
            })
        } else {
            Err("Age must be at least 21")
        }                                                //Self is a check if its an adult.

    }
}

fn main() {
    let child = Adult::new(15, "Anita");
    let adult = Adult::new(21, "Sanjay");
match child {
        Ok(child) => println!("{} is {} years old", child.name, child.age),
        Err(e) => println!("{e}"),     
    }
match adult {
        Ok(adult) => println!("{} is {} years old", adult.name, adult.age),
        Err(e) => println!("{e}"),     
    }
    
}
