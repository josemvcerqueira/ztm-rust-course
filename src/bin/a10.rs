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

fn handle_is_big(x: bool) {
    match x {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn main() {
    let num = 99;
    let is_big = num > 100;

    handle_is_big(is_big);
}
