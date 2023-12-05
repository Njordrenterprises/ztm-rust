// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

//-----> first try 
// fn main() {   
// // * Use a function that returns a tuple
//     fn cart_check(){
//         let coord = (x, y);
//         return coord;
//     }    

// // * Destructure the return value into two variables

//     let (">", "<") = (coord.0, coord.1);

// // * Use an if..else if..else block to determine what to print
//     if coord.1 > 5 => println!({?}, coord.0);
// }   
// }
// //----->NOT TODAY


// * Use a function that returns a tuple
fn coordinate() -> (i32, i32) {
    (1, 7)
}
fn main() {
// * Destructure the return value into two variables
    let (_x, _y) = coordinate();
// * Use an if..else if..else block to determine what to print
    if _y > 5 {
        println!(">5");
    } else if _y < 5 {
        println!("<5");
    } else {
        println!("=5");
    }
}
