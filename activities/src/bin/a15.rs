// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info


// * Use an enum for the tickets with data associated with each variant

//first try
// Struct TicketData {
//     price: f32,
//       first_name: string,
//       last_name: string, 
// }

// enum Tickets { 
//     Standard,
//     Backstage,
//     Vip,
// }

// enum TicketData {
//  
//     }
// }
// fn main() {}

// // meh not today

// * Tickets can be Backstage, Vip, and Standard
// * All tickets include the price
enum Ticket {
    Backstage(f64, String), //variant
    Standard(f64), //variant
    Vip(f64, String)//variant 
//for all variants you can't omit any data types if you call it. 
}
 
fn main() {
    //* Create one of each ticket and place into a vector
    let tickets = vec![ //this vector is a collection
        Ticket::Backstage(50.0, "Billy".to_owned()),//::double colons access the enum variants
        Ticket::Standard(15.0),
        Ticket::Vip(30.0, "Terry".to_owned()),
    ];
// * Use a match expression while iterating the vector to print the ticket info
    for ticket in tickets { //for each 'thing' inside a 'collection'
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Holder: {:?}, price: {:?}", holder, price)//can do multiple
                //things inside of a match arm but putting each arm inside the {}
            }
            Ticket::Standard(price) => println!("price: {:?}",price),
            Ticket::Vip(price, holder) => println!("Holder: {:?}, price: {:?}", holder, price),
        }
    }
}
