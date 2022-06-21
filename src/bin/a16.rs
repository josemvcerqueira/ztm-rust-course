// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Assignment {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student_assignment = Assignment {
        name: "jose".to_owned(),
        locker: Some(32),
    };

    println!("The student name is {:?}", student_assignment.name);

    match student_assignment.locker {
        Some(combination) => println!("His locker combination is {:?}", combination),
        None => print!("This student has no locker"),
    }
}
