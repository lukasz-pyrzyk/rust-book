use std::io;

fn main() {
    println!("Provide a temperature in Fahrenheit: ");
    let mut fehrenheit = String::new();

    io::stdin().read_line(&mut fehrenheit).expect("Failed to temperature input");
    let fehrenheit: i32 = fehrenheit.trim().parse().expect("Failed to parse input");

    let celsius = (fehrenheit - 32) * 5 / 9;
    println!("The temperature in Celsius is: {celsius}");
}
