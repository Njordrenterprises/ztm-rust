// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// //first try 
// // * Use a struct for a persons age, name, and favorite color
// struct Person {
//     age: i32,
//     name: string,
//     fav_color: string,
// }

// // * Create and store at least 3 people in a vector
// Const = vec![
//     Line Item {
//     name: Bill,
//     age: 35,
//     fav_color: red,
//     },
//     Line Item {
//     name: Ted,
//     age: 42,
//     fav_color: blue,
//     },
//     Line Item {
//     name: Bill,
//     age: 17,
//     fav_color: yellow,
//     },
// ];

// fn main() {}


// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    name: String,
    fav_color: String,
    age: i32
}

fn print(data: &str) { //You use a string slice &str when you 
    //just need to read from a string or pass it around without 
    //needing ownership. It's more efficient in cases where 
    //you don't need to change the string.
    println!("Name and Favorite Color: {:?}", data);
}
// additional function, so I can print the age with an int
fn print_int(data: &i32) {
    println!("That guys age is: {:?}", data);
}

// * The name and colors should be printed using a function

fn main() {
// * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            name: String::from("George"),
            fav_color: String::from("green"),
            age: 7,
        },
 
        Person {
            name: String::from("Ted"),
            fav_color: String::from("yellow"),
            age: 9,
        },
        Person {
            name: String::from("George"),
            fav_color: String::from("green"),
            age: 14,
        },
    ];
// * Iterate through the vector using a for..in loop
for person in &people {
// * Use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            print(&person.name);
            print(&person.fav_color);
            print_int(&person.age);
// * The name and colors should be printed using a function

        }
    }
}
