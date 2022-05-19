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

fn coordinate() -> (i32, i32) {
    (10, 4)
}

fn handle_y(y: i32) {
    if y > 5 {
        println!("greater than 5");
        return;
    }

    if y < 5 {
        println!("less than 5");
        return;
    }

    println!("equal to 5");
}

fn main() {
    let (_, y) = coordinate();

    handle_y(y);
}
