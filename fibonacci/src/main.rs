use std::io;

fn main() {
    println!("Give N for fibonacci generator: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read input");

    let n: u32 = input.trim().parse().expect("Please enter a number");

    let result = fibonacci(n);
    println!("Fibonacci of n {n} is {result}")
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            let mut next;
            for _ in 2..=n {
                next = a + b;
                a = b;
                b = next;
            }

            b
        }
    }
}