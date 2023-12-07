// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

//first try
// enum BoxColor {
//     blue,
//     red,
//     white,
//     black,
// } 
// struct BoxChar {
//     dimensions: f64,
//     weight: f64,
//     color: BoxColor{
//     match BoxColor {

//     }
// } 
// }


//too tired today!



// * Use an enum for the box color
// Define an enum named 'Color' with variants 'Brown' and 'Red'.
// Enums in Rust are types which can be one of a few different variants.
enum Color {
    Brown,
    Red
}

// Implement methods for the 'Color' enum.
impl Color {
    // Method to print the color.
    // '&self' means we're borrowing 'Color' for reading, not modifying it.
    fn print(&self) {
        match self {
            // If the color is 'Brown', print "brown".
            Color::Brown => println!("brown"),
            // If the color is 'Red', print "red".
            Color::Red => println!("red"),
        }
    }
}

// Define a struct named 'Dimensions' to hold the dimensions of an object.
// Structs in Rust are used to create custom data types.
struct Dimensions {
    width: f64,   // f64 is a floating-point type. This represents width.
    height: f64,  // Represents height.
    depth: f64,   // Represents depth.
}

// Implement methods for the 'Dimensions' struct.
impl Dimensions {
    // Method to print the dimensions.
    fn print(&self) {
        // Print each dimension on a new line.
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

// Define a struct named 'ShippingBox'.
struct ShippingBox {
    color: Color,         // 'Color' enum to represent the color of the box.
    weight: f64,          // f64 to represent the weight of the box.
    dimensions: Dimensions, // 'Dimensions' struct to represent the size of the box.
}

// Implement methods for 'ShippingBox'.
impl ShippingBox {
    // Constructor method to create a new 'ShippingBox'.
    // Takes weight (f64), color (Color), and dimensions (Dimensions) as arguments.
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,        // Set the weight of the box.
            color,         // Set the color of the box.
            dimensions,    // Set the dimensions of the box.
        }
    }

    // Method to print the details of the box.
    fn print(&self) {
        // Print the color of the box.
        self.color.print();
        // Print the dimensions of the box.
        self.dimensions.print();
        // Print the weight of the box.
        println!("weight: {:?}", self.weight);
    }
}

// Main function - the entry point of the program.
fn main() {
    // Create an instance of 'Dimensions' named 'small_dimensions'.
    let small_dimensions = Dimensions {
        width: 1.0,   // Set width to 1.0.
        height: 2.0,  // Set height to 2.0.
        depth: 3.0,   // Set depth to 3.0.
    }; // Always end let bindings with a semi-colon.

    // Create a new 'ShippingBox' called 'small_box'.
    let small_box = ShippingBox::new(5.0, Color::Red, small_dimensions);
    // Call the print method to print details of 'small_box'.
    small_box.print();
}
