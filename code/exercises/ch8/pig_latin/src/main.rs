use std::io;

fn main() {
    // Read input from the user
    println!("Enter words separated by whitespace:");
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read input");

    // Remove the trailing newline character
    text = text.trim_end().to_string();

    // Convert the input text to Pig Latin
    let pig_latin = pig_latin::convert_to_pig_latin(&text);

    // Display the original text and the Pig Latin text
    println!("\nOriginal: {}", text);
    println!("Pig Latin: {}", pig_latin);
}
