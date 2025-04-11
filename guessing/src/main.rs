use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {

    let secret_number = get_random(1, 100);

    loop {
        println!("Guess the number!");
        let mut guess = String::new();

        io::stdin().read_line( & mut guess).expect("Failed to read line from user");

        let guess : u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => println!("You guessed it!"),
            Ordering::Greater => println!("Too big!"),
        }
    }
}

fn get_random(from: u32, to: u32) -> u32 {
    rand::thread_rng().gen_range(from..=to)
}
