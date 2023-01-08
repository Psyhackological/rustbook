use temp_converter::{Celsius, Kelvin};

fn main() {
    let kelvin = Kelvin::new(273.15);
    println!("{:?}", kelvin);

    let celsius = Celsius::from(kelvin);
    println!("{:?}", celsius);
}
