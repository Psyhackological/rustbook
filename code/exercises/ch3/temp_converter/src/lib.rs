pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Temperature {
    pub fn convert(&self) -> Self {
        match self {
            Self::Celsius(c) => {
                let f = c * 9.0 / 5.0 + 32.0;
                Temperature::Fahrenheit(f)
            }
            Self::Fahrenheit(f) => {
                let c = (f - 32.0) * 5.0 / 9.0;
                Temperature::Celsius(c)
            }
        }
    }

    #[allow(dead_code)]
    fn inner(&self) -> f64 {
        match self {
            Self::Celsius(c) => *c,
            Self::Fahrenheit(f) => *f,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Temperature::*;

    const TOLERANCE: f64 = 0.01;

    fn float_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    #[test]
    fn absolute_zero() {
        let absolute_zero_f = Fahrenheit(-459.67);
        let absolute_zero_c = Celsius(-273.15);

        let converted_absolute_zero_f = absolute_zero_f.convert();
        let converted_absolute_zero_c = absolute_zero_c.convert();

        assert!(float_eq(
            converted_absolute_zero_f.inner(),
            absolute_zero_c.inner(),
            TOLERANCE
        ));
        assert!(float_eq(
            converted_absolute_zero_c.inner(),
            absolute_zero_f.inner(),
            TOLERANCE
        ));
    }

    #[test]
    fn zero() {
        let zero_f = Fahrenheit(0.0);
        let zero_c = Celsius(-17.78);

        let converted_zero_f = zero_f.convert();
        let converted_zero_c = zero_c.convert();

        assert!(float_eq(
            converted_zero_f.inner(),
            zero_c.inner(),
            TOLERANCE
        ));
        assert!(float_eq(
            converted_zero_c.inner(),
            zero_f.inner(),
            TOLERANCE
        ));
    }

    #[test]
    fn freezing_point() {
        let freezing_f = Fahrenheit(32.0);
        let freezing_c = Celsius(0.0);

        let converted_freezing_f = freezing_f.convert();
        let converted_freezing_c = freezing_c.convert();

        assert!(float_eq(
            converted_freezing_f.inner(),
            freezing_c.inner(),
            TOLERANCE
        ));
        assert!(float_eq(
            converted_freezing_c.inner(),
            freezing_f.inner(),
            TOLERANCE
        ));
    }

    #[test]
    fn body_temperature() {
        let body_f = Fahrenheit(98.6);
        let body_c = Celsius(37.0);

        let converted_body_f = body_f.convert();
        let converted_body_c = body_c.convert();

        assert!(float_eq(
            converted_body_f.inner(),
            body_c.inner(),
            TOLERANCE
        ));
        assert!(float_eq(
            converted_body_c.inner(),
            body_f.inner(),
            TOLERANCE
        ));
    }

    #[test]
    fn boiling_point() {
        let boiling_f = Fahrenheit(212.0);
        let boiling_c = Celsius(100.0);

        let converted_boiling_f = boiling_f.convert();
        let converted_boiling_c = boiling_c.convert();

        assert!(float_eq(
            converted_boiling_f.inner(),
            boiling_c.inner(),
            TOLERANCE
        ));
        assert!(float_eq(
            converted_boiling_c.inner(),
            boiling_f.inner(),
            TOLERANCE
        ));
    }
}
