use std::time::SystemTime;

const MAX_LIMIT: u32 = 1_000_000;

fn main() {
    // Question: Is "1"(b) and "1"(decimal) a palindrome or not?
    let mut number = 0;
    let mut sum = 0;

    let benchmark_start = SystemTime::now();

    while number != MAX_LIMIT {
        number += 1;

        if is_palindrome(&format!("{}", number)) {
            let binary_represenation = format!("{:b}", number);

            if is_palindrome(&binary_represenation) {
                sum += number;
                println!("Palindrom: {}, 0b{}", number, binary_represenation);
            }
        }
    }

    let benchmark_duration = SystemTime::now().duration_since(benchmark_start).unwrap();

    println!("Sum: {}", sum);
    println!("Took: {:?}", benchmark_duration);
}

#[inline(always)]
fn is_palindrome(number: &str) -> bool {
    number.chars().rev().collect::<Vec<_>>() == number.chars().collect::<Vec<_>>()
}
