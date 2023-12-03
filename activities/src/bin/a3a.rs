// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal


// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"

//first attempt
// fn main() { 
//     let a = 1;
//     if a == 1 {
//     a = true;
//         println!("Hello");

//     } else if a !=1 {
//         a = false; {
//         println!("Goodbye");
//         }
//     }
// }
//

 // * Displays a message based on the value of a boolean variable
fn main() {
   let a = false;
 // * When the variable is set to true, display "hello"
    if a == true {
        println!("Hello");
    } else {
        println!("goodbye");
    }
}
