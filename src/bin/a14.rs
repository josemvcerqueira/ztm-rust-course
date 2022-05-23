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

struct Person {
    color: String,
    name: String,
    age: i32,
}

fn print_name(name: &str) {
    println!("person name is {:?}", name);
}

fn print_color(color: &str) {
    println!("person favorite color is {:?}", color);
}

fn main() {
    let people = vec![
        Person {
            color: "blue".to_owned(),
            age: 29,
            name: "Pedro".to_owned(),
        },
        Person {
            color: "pink".to_owned(),
            age: 28,
            name: "Maesto".to_owned(),
        },
        Person {
            color: "black".to_owned(),
            age: 30,
            name: "Jose".to_owned(),
        },
    ];

    for person in people {
        print_name(&person.name);
        print_color(&person.color);
        println!("person is {:?} years old", person.age);
    }
}
