use std::io;

fn main() {
    println!(
        "This program can helps you to convert temperatures between Fahrenheit and Celsius.\n"
    );
    println!("What units do you want to convert to others?\n");
    println!("Type one of following unit below:");
    println!("    f - convert Fahrenheit to Celsius;");
    println!("    c - convert Celsius to Fahrenheit;");

    let mut unit = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    if unit.trim().to_lowercase() == "f" {
        println!("You pick Fahrenheit: {}", unit);
    } else if unit.trim().to_lowercase() == "c" {
        println!("You pick Celsius: {}", unit);
    }

    println!("Now please input value to convert:");

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value: f32 = value.trim().parse().expect("incorrect value");

    println!("You enter a value: {}", value);

    if unit.trim().to_lowercase() == "f" {
        let result = ((value + 40 as f32) / 1.8) - 40 as f32;

        println!("{} degrees Fahrenheit = {} degrees Celsius", value, result);
    } else if unit.trim().to_lowercase() == "c" {
        let result = ((value + 40 as f32) * 1.8) - 40 as f32;

        println!("{} degrees Celsius = {} degrees Fahrenheit", value, result);
    }
}
