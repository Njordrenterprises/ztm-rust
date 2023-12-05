// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

//first try 
// enum var_size {
//     big,
//     small
// };

// fn main() {
//     let num_var = 5;
//  let var_accept = check_var_size {
//         if num_var > 100 -> true;
//     } else num_var < 100 -> false;

// fn print_size() {
//     println!(":?", check_var_size)
// }

// }

//not quite...

fn print_message(gt_100: bool){
    match gt_100 {
        true => println!("its big"),
        false => println!("its small"),
    }
}
fn main() {
    // boolean to  expression
    let value = 100;
    let is_gt_100 = value > 100;//let can be used in place of logical "is"
    print_message(is_gt_100);
   }
