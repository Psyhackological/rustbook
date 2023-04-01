use std::io::{self, Write};

enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Temperature {
    fn convert(&self) -> Self {
        match self {
            Temperature::Celsius(c) => {
                let f = c * 9.0 / 5.0 + 32.0;
                Temperature::Fahrenheit(f)
            }
            Temperature::Fahrenheit(f) => {
                let c = (f - 32.0) * 5.0 / 9.0;
                Temperature::Celsius(c)
            }
        }
    }
}

fn main() {
    print!("Enter the temperature value followed by its unit (C or F): ");
    io::stdout().flush().unwrap(); // Flush stdout to make sure the prompt is shown before read_line

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let (value, unit) = input.split_at(input.trim_end().len() - 1);
    let value: f64 = value.parse().expect("Failed to parse temperature value");
    let unit = unit.trim().to_uppercase();

    let temp = match unit.as_str() {
        "C" => Temperature::Celsius(value),
        "F" => Temperature::Fahrenheit(value),
        _ => {
            println!("Invalid temperature unit. Use 'C' for Celsius or 'F' for Fahrenheit.");
            return;
        }
    };

    let converted_temp = temp.convert();
    match (temp, converted_temp) {
        (Temperature::Celsius(c), Temperature::Fahrenheit(f)) => {
            println!("{:.2}°C is {:.2}°F", c, f)
        }
        (Temperature::Fahrenheit(f), Temperature::Celsius(c)) => {
            println!("{:.2}°F is {:.2}°C", f, c)
        }
        _ => unreachable!(),
    }
}
