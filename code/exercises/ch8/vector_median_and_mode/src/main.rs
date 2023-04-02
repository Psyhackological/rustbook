use std::io;
use vector_median_and_mode::{calculate_median, calculate_mode};

fn main() {
    println!("Enter numbers separated by spaces or commas:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let nums: Vec<i32> = input
        .split(|c| c == ',' || c == ' ')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let median = calculate_median(&nums);
    let mode = calculate_mode(&nums);

    println!("Your numbers {:?}", nums);
    println!("Median = {}", median);
    match mode {
        Some(value) => println!("Mode = {}", value),
        None => println!("Mode = All numbers occur the same number of times."),
    }
}
