use fibonacci::Fibonacci;

fn main() {
    println!("Enter the position (n) of the Fibonacci number: ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let n: usize = input
        .trim()
        .parse()
        .expect("Failed to parse the input as an integer");

    let fib_n = Fibonacci::new().nth(n);
    println!("The {}-th Fibonacci number is: {}", n, fib_n);
}
