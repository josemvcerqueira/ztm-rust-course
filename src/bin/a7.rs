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
    Orange,
    Pink,
    Red,
}

fn print_color(color: Color) {
    match color {
        Color::Blue => println!("blue"),
        Color::Orange => println!("orange"),
        Color::Pink => println!("pink"),
        Color::Red => println!("red"),
    }
}

fn main() {
    let blue = Color::Blue;
    let orange = Color::Orange;
    let pink = Color::Pink;
    let red = Color::Red;

    print_color(blue);
    print_color(orange);
    print_color(pink);
    print_color(red);
}
