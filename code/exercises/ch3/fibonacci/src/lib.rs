use num_bigint::BigUint;
use num_traits::{One, Zero};

// The matrix exponentiation method relies on the following relation:
// | F(n+1) |   | 1 1 |^n   | F(1) |
// | F(n)   | = | 1 0 |   * | F(0) |
pub struct Fibonacci;

// Implement the Default trait for easier initialization.
impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Fibonacci {
    // Create a new Fibonacci struct.
    pub fn new() -> Self {
        Fibonacci
    }

    // Compute the nth Fibonacci number using the matrix exponentiation method.
    pub fn nth(&self, n: usize) -> BigUint {
        if n == 0 {
            return BigUint::zero();
        }

        // Initialize the 2x2 Fibonacci matrix as a flat array with 4 elements.
        let mut matrix = [
            BigUint::one(),
            BigUint::one(),
            BigUint::one(),
            BigUint::zero(),
        ];

        // Compute the matrix raised to the power (n - 1).
        self.matrix_pow(&mut matrix, n - 1);

        // Return the top-left element, which is the nth Fibonacci number.
        matrix[0].clone()
    }

    // Multiply two 2x2 matrices of BigUint elements.
    // Takes two flat arrays of length 4 as input.
    fn matrix_mult(&self, a: &[BigUint; 4], b: &[BigUint; 4]) -> [BigUint; 4] {
        // Compute the elements of the resulting matrix using matrix multiplication.
        let a00 = &a[0] * &b[0] + &a[1] * &b[2];
        let a01 = &a[0] * &b[1] + &a[1] * &b[3];
        let a10 = &a[2] * &b[0] + &a[3] * &b[2];
        let a11 = &a[2] * &b[1] + &a[3] * &b[3];

        // Return the result as a new flat array.
        [a00, a01, a10, a11]
    }

    // Compute the matrix raised to the power n using the binary exponentiation method.
    // Takes a mutable reference to a flat array of length 4 and an integer power n.
    fn matrix_pow(&self, matrix: &mut [BigUint; 4], n: usize) {
        if n == 0 || n == 1 {
            return;
        }

        // Make a copy of the matrix.
        let mut temp_matrix = matrix.clone();

        // Compute the matrix raised to the power (n / 2) recursively.
        self.matrix_pow(&mut temp_matrix, n / 2);

        // Square the matrix.
        let temp_matrix_squared = self.matrix_mult(&temp_matrix, &temp_matrix);

        // If n is even, the result is the squared matrix.
        if n % 2 == 0 {
            *matrix = temp_matrix_squared;
        } else {
            // If n is odd, the result is the product of the squared matrix and the original matrix.
            let fib_matrix = [
                BigUint::one(),
                BigUint::one(),
                BigUint::one(),
                BigUint::zero(),
            ];
            *matrix = self.matrix_mult(&temp_matrix_squared, &fib_matrix);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a new Fibonacci instance
    fn create_fib(n: usize) -> BigUint {
        Fibonacci::new().nth(n)
    }

    #[test]
    fn fib_0() {
        let fib_0 = create_fib(0);
        assert_eq!(fib_0, BigUint::from(0u32), "F(0) should be 0");
    }

    #[test]
    fn fib_1_and_2() {
        let fib_1 = create_fib(1);
        assert_eq!(fib_1, BigUint::from(1u32), "F(1) should be 1");

        let fib_2 = create_fib(2);
        assert_eq!(fib_2, BigUint::from(1u32), "F(2) should be 1");

        assert_eq!(fib_1, fib_2, "F(1) and F(2) should both be 1");
    }

    #[test]
    fn fib_7() {
        let fib_7 = create_fib(7);
        assert_eq!(fib_7, BigUint::from(13u32), "F(7) should be 13");
    }

    #[test]
    fn fib_77() {
        let fib_77 = create_fib(77);
        assert_eq!(
            fib_77,
            BigUint::parse_bytes(b"5527939700884757", 10).unwrap(),
            "The 77th Fibonacci number should be 5527939700884757"
        );
    }

    #[test]
    fn fib_777() {
        let fib_777 = create_fib(777);
        assert_eq!(
            fib_777,
            BigUint::parse_bytes(
                b"1081213530912648191985419587942084110095342850438593857649766278346130479286685742885693301250359913460718567974798268702550329302771992851392180275594318434818082",
                10
            )
            .unwrap(),
            "The 777th Fibonacci number should be 1081213530912648191985419587942084110095342850438593857649766278346130479286685742885693301250359913460718567974798268702550329302771992851392180275594318434818082"
        );
    }
}
