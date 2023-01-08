use std::convert::From;

#[derive(Debug, PartialEq)]
struct Kelvin(f32);

#[derive(Debug, PartialEq)]
struct Celsius(f32);

impl Kelvin {
    fn new(temp: f32) -> Self {
        if temp >= 0.00 && temp <= 1273.15 {
            Self(temp)
        } else {
            panic!("Celsius should be between 0 K and 1273.15 K");
        }
    }

    fn min() -> Self {
        Self(0.0)
    }

    fn max() -> Self {
        Self(1273.15)
    }

    fn inner(&self) -> f32 {
        self.0
    }
}

impl Celsius {
    fn new(temp: f32) -> Self {
        if temp >= -273.15 && temp <= 726.85 {
            return Self(temp);
        } else {
            panic!("Celsius should be between -273.15 C and 726.85 C");
        }
    }

    fn min() -> Self {
        Self(-273.15)
    }

    fn max() -> Self {
        Self(1000.0)
    }

    fn inner(&self) -> f32 {
        self.0
    }
}

impl From<Kelvin> for Celsius {
    fn from(kelvin: Kelvin) -> Self {
        Self(kelvin.0 - 273.15)
    }
}

impl From<Celsius> for Kelvin {
    fn from(celcius: Celsius) -> Self {
        Self(celcius.0 + 273.15)
    }
}

// impl PartialEq<Kelvin> for Celsius {
//     fn eq(&self, &kelvin: Kelvin) -> bool {
//         (kelvin.0 - 273.15) == self.0
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_from_conversion() {
        let to_celcius = Celsius::from(Kelvin::max());
        let to_kelvin = Kelvin::from(Celsius::max());

        assert_eq!(to_celcius.inner(), Celsius::max().inner());
        assert_eq!(to_kelvin.inner(), Kelvin::max().inner());
    }

    #[test]
    fn min_from_conversion() {
        let to_celcius = Celsius::from(Kelvin::min());
        let to_kelvin = Kelvin::from(Celsius::min());

        assert_eq!(to_celcius.inner(), Celsius::min().inner());
        assert_eq!(to_kelvin.inner(), Kelvin::min().inner());
    }

    #[test]
    fn general_conversion() {
        let room_temperature_c = Celsius::new(21.00);
        let room_temperature_k = Kelvin::new(294.15);

        let to_c = Celsius::from(room_temperature_k);
        let to_k = Kelvin::from(room_temperature_c);

        assert_eq!(to_c.inner(), 21.00);
        assert_eq!(to_k.inner(), 294.15);
    }
}
