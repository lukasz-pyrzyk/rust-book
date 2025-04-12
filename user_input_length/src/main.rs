use std::io;

fn main() {

    println!("Please enter a string: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let length = calculate_length(&input);
    println!("The length of the input is: {length}");
}

fn calculate_length(input: &String) -> usize {
    input.trim().len()
}
