pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Temperature {
    pub fn convert(&self) -> Self {
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
