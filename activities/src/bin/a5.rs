// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
let mut start = 1;
    loop {
        println!("{:?}", start);
    start = start + 1;
        if start == 5 {
            break;
        }
        // start = start + 1; 
        // class solution places increment here.  
        // why?
    }
}
