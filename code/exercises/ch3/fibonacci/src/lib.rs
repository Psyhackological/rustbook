use num_bigint::BigUint;
use num_traits::{One, Zero};

// The matrix exponentiation method relies on the following relation:
// | F(n+1) |   | 1 1 |^n   | F(1) |
// | F(n)   | = | 1 0 |   * | F(0) |
pub struct Fibonacci;

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci
    }

    // Compute the nth Fibonacci number using the matrix exponentiation method.
    pub fn nth(&self, n: usize) -> BigUint {
        if n == 0 {
            return BigUint::zero();
        }

        // Initialize the Fibonacci matrix.
        let mut matrix = vec![
            vec![BigUint::one(), BigUint::one()],
            vec![BigUint::one(), BigUint::zero()],
        ];

        // Compute the matrix raised to the power (n - 1).
        self.matrix_pow(&mut matrix, n - 1);
        matrix[0][0].clone()
    }

    // Multiply two matrices of BigUint elements.
    fn matrix_mult(&self, a: &[Vec<BigUint>], b: &[Vec<BigUint>]) -> Vec<Vec<BigUint>> {
        let a00 = &a[0][0] * &b[0][0] + &a[0][1] * &b[1][0];
        let a01 = &a[0][0] * &b[0][1] + &a[0][1] * &b[1][1];
        let a10 = &a[1][0] * &b[0][0] + &a[1][1] * &b[1][0];
        let a11 = &a[1][0] * &b[0][1] + &a[1][1] * &b[1][1];

        // Return the result as a new matrix.
        vec![vec![a00, a01], vec![a10, a11]]
    }

    // Compute the matrix raised to the power n using the binary exponentiation method.
    fn matrix_pow(&self, matrix: &mut Vec<Vec<BigUint>>, n: usize) {
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
            let fib_matrix = vec![
                vec![BigUint::one(), BigUint::one()],
                vec![BigUint::one(), BigUint::zero()],
            ];
            *matrix = self.matrix_mult(&temp_matrix_squared, &fib_matrix);
        }
    }
}
