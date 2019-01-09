#![feature(test)]

#[cfg(test)]
extern crate test;

use std::time::SystemTime;

fn main() {
    let benchmark_start = SystemTime::now();
    let result = greatest_palindrome_number(3);

    let benchmark_duration = SystemTime::now().duration_since(benchmark_start).unwrap();

    println!(
        "Largest palindrome number of three digit multiplicants: {} (took {:?})",
        result, benchmark_duration
    );
}

fn greatest_palindrome_number(max_digits: u32) -> u64 {
    assert!(max_digits > 0);

    let mut i = 0;
    let j = 10_u64.pow(max_digits) - 1;

    let mut greatest_number_found = 0;

    while get_digits(i) < max_digits {
        i += 1;

        let product_result = i * j;

        // save further calculations when this number is smaller or equal than the one we already found.
        if product_result <= greatest_number_found {
            continue;
        }

        let product_result_string = format!("{}", product_result);

        // impossible case, as the numbers start with no leading zero(s).
        if product_result_string.ends_with('0') {
            continue;
        }

        if product_result_string.chars().collect::<String>()
            == product_result_string.chars().rev().collect::<String>()
        {
            println!("Found new greatest number: {}", product_result);
            greatest_number_found = product_result;
        }
    }

    greatest_number_found as u64
}

fn get_digits(number: u64) -> u32 {
    ((number as f64).log10().floor() + 1_f64) as u32
}

#[cfg(test)]
mod tests {
    use super::{get_digits, greatest_palindrome_number};
    use test::{black_box, Bencher};

    #[test]
    fn test_two_digit_palindrome() {
        assert_eq!(greatest_palindrome_number(2), 9009);
    }

    #[test]
    fn test_get_digits() {
        assert_eq!(get_digits(10), 2);
        assert_eq!(get_digits(300), 3);
        assert_eq!(get_digits(342), 3);
        assert_eq!(get_digits(100000), 6);
    }

    #[bench]
    fn bench_greatest_palindrom_number(b: &mut Bencher) {
        b.iter(|| {
            black_box(greatest_palindrome_number(6));
        });
    }

}
