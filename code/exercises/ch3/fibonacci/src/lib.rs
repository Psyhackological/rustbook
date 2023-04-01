pub struct Fibonacci {
    prev_prev: u64,
    prev: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci {
            prev_prev: 0,
            prev: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.prev_prev + self.prev;
        self.prev_prev = self.prev;
        self.prev = current;

        Some(self.prev_prev)
    }
}
