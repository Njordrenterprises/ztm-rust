// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

//first try - boolean man... read the instructions
// fn main() {
//     let a = 5;
//     match a {
//         1 => println!("YES!"),
//         _ => println!("NO!"),
//     }    
//     
// }

fn main() {
let a = false;
match a {
    true => println!("TRUE"),
    false => println!("FALSE"),
    }
}
