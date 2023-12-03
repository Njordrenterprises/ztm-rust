// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

//first attempt
// fn main() {
//     let mut a = 5; 
//     while a <= 5 {
//         println!("{:?}", a);
//         a = a - 1;
//     
//     if a == 0 => {
//         println!("DONZO");
//         break;
//     }
// }

fn main() {
    let mut a = 5;
    while a >= 1 {
        println!("{:?}", a);
        a = a - 1;
    }
    println!("DONZO");
}

