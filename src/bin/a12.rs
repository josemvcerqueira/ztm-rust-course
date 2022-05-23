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

enum Color {
    Black,
    White,
    Brown,
}

struct ShippingBox {
    dimensions: i32,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    fn black_box() -> Self {
        Self {
            dimensions: 64,
            weight: 11.5,
            color: Color::Black,
        }
    }

    fn print_dimensions(&self) {
        println!("Box has a dimension of {:?}", self.dimensions);
    }

    fn print_color(&self) {
        match self.color {
            Color::Black => println!("Box is black"),
            Color::White => println!("Box is white"),
            Color::Brown => println!("Box is brown"),
        }
    }

    fn print_weight(&self) {
        println!("Box weighs {:?}", self.weight);
    }
}

fn main() {
    let black_box = ShippingBox::black_box();
    black_box.print_color();
    black_box.print_weight();
    black_box.print_dimensions();
}
