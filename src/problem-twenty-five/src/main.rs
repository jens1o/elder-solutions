struct Fibonacci {
    index: u32,
    cache: Vec<u128>,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci {
            index: 0,
            cache: vec![1, 1],
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u128;

    /// yields the results starting with F(3) = 2. F(1) and F(2) are omitted!
    fn next(&mut self) -> Option<u128> {
        self.index += 1;

        let result =
            (self.cache[(self.index - 1) as usize] + self.cache[(self.index) as usize]) as u128; // FIXME: This overflows at index of 183.
        self.cache.push(result);

        Some(result)
    }
}

fn main() {
    let fibonacci = Fibonacci::new();

    for (x, fibonacci) in fibonacci.enumerate() {
        let digit_count = (fibonacci as f64).log10().floor() + 1_f64;

        println!(
            "fibonacci of {} is {} ({} digits)",
            x + 3,
            fibonacci,
            digit_count
        );

        if digit_count == 1000_f64 {
            println!(
                "The first fibonacci number with 1000 digits is {} ({})",
                x + 3,
                fibonacci
            );
        }
    }
}
