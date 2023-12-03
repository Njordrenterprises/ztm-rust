// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print


enum Color {
    Blue,
    Red,
    Green,
    Yellow,
}

fn color_print(the_color: Color) {
    match the_color {
        Color::Blue => println!("Blue"),
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Yellow => println!("Yellow"),
    }
}

fn main() {
    color_print(Color::Blue);
}
