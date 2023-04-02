use department::Company;
use std::io::{self, Write};

fn main() {
    let mut company = Company::new();

    loop {
        print!("Please enter a command (type 'exit' to quit): ");
        io::stdout().flush().unwrap(); // Flush stdout to make sure the prompt is shown before read_line

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let output = company.process_input(input);
        println!("{}", output);
    }
}
