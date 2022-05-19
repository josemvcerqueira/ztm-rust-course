// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Vanilla,
    Orange,
    Ginger,
}

struct Drink {
    flavor: Flavor,
    ounces: f64,
}

fn handle_flavor(flavor: Flavor) -> &'static str {
    match flavor {
        Flavor::Vanilla => return "vanilla",
        Flavor::Orange => return "orange",
        Flavor::Ginger => return "ginger",
    }
}

fn print_drink(drink: Drink) {
    println!(
        "This {:?} drink has {:?} ounces",
        handle_flavor(drink.flavor),
        drink.ounces
    );
}

fn main() {
    let drink = Drink {
        ounces: 4.99,
        flavor: Flavor::Vanilla,
    };

    print_drink(drink);
}
