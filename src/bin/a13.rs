// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn print_num(&num: &i32) {
    match num {
        30 => println!("thirty"),
        _ => println!("{:?}", num),
    }
}

fn print_len(vec: Vec<i32>) {
    println!("This vector has a length of {:?}", vec.len());
}

fn main() {
    let nums = vec![10, 20, 30, 40];

    for num in &nums {
        print_num(&num);
    }

    print_len(nums);
}
