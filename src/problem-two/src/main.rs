use std::env;
use std::time::SystemTime;

const FIBONACCI_LIMIT: usize = 4_000_000;

fn fibonacci(number: usize) -> usize {
    let mut current_number: usize = 0;
    let mut fibonacci_numbers: Vec<usize> = Vec::with_capacity(number);

    while current_number != number {
        if current_number < 2 {
            fibonacci_numbers.push(1);
        } else {
            let sum =
                fibonacci_numbers[(current_number - 2)] + fibonacci_numbers[(current_number - 1)];

            if sum >= FIBONACCI_LIMIT {
                panic!(
                    "Hit max-limit of {} while calculating fibonacci sum of even numbers.",
                    FIBONACCI_LIMIT
                );
            }

            fibonacci_numbers.push(sum);
        }
        current_number += 1;
    }

    fibonacci_numbers.into_iter().filter(|x| x % 2 == 0).sum()
}

fn main() {
    let number: usize = env::args()
        .nth(1)
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(33); // 33 is enough to not go beyond the fibonacci limit of 4,000,000.

    let benchmark_start = SystemTime::now();

    let result = fibonacci(number);

    // unwrap is safe because code-wise benchmark start is earlier
    let benchmark_duration = SystemTime::now().duration_since(benchmark_start).unwrap();

    println!(
        "The sum of the even numbers in the term of finding the Fibonacci-Number for {} is: {} (took {:?})",
        number,
        result,
        benchmark_duration
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(10), 44);
    }

    #[test]
    fn test_sum_of_less_than_4_million() {
        assert_eq!(fibonacci(33), 4613732);
    }
}
