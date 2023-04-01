use fibonacci::Fibonacci;
use std::io::{self, Write};

fn main() {
    print!("Enter the position (n) of the Fibonacci number: ");
    io::stdout().flush().unwrap(); // Flush stdout to make sure the prompt is shown before read_line

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let n: usize = input
        .trim()
        .parse()
        .expect("Failed to parse the input as an integer");

    let fib_n = Fibonacci::new().nth(n).unwrap();
    match n {
        1 => println!("The {}-st Fibonacci number is: {}", n, fib_n),
        2 => println!("The {}-nd Fibonacci number is: {}", n, fib_n),
        3 => println!("The {}-rd Fibonacci number is: {}", n, fib_n),
        _ => println!("The {}-th Fibonacci number is: {}", n, fib_n),
    }
}
