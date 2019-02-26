use std::env;
use std::time::SystemTime;

fn main() {
    let i: u64 = env::args()
        .nth(1)
        .and_then(|x| x.parse::<_>().ok())
        .unwrap_or(100);

    let benchmark_start = SystemTime::now();
    let difference =
        square_sum_of_numbers_less_than_i(i) - sum_of_square_numbers_of_numbers_less_than_i(i);
    let benchmark_end = SystemTime::now().duration_since(benchmark_start).unwrap();

    println!(
        "Difference between square-sum-of-numbers and sum-of-square-numbers is: {}",
        difference
    );

    println!("Calucation took {:?}.", benchmark_end);
}

fn sum_of_square_numbers_of_numbers_less_than_i(i: u64) -> u64 {
    assert!(i != 0);
    let mut result: u64 = 0;

    for number in 0..=i {
        result += number.pow(2);
    }

    result
}

fn square_sum_of_numbers_less_than_i(i: u64) -> u64 {
    assert!(i != 0);
    let mut result: u64 = 0;

    for number in 0..=i {
        result += number;
    }

    result.pow(2)
}

#[cfg(test)]
mod tests {
    use super::{square_sum_of_numbers_less_than_i, sum_of_square_numbers_of_numbers_less_than_i};

    #[test]
    fn test_sum_of_square_numbers() {
        assert_eq!(sum_of_square_numbers_of_numbers_less_than_i(10), 385);
    }

    #[test]
    fn test_square_sum_of_numbers() {
        assert_eq!(square_sum_of_numbers_less_than_i(10), 3025);
    }

    #[test]
    fn test_difference() {
        assert_eq!(
            square_sum_of_numbers_less_than_i(10)
                - sum_of_square_numbers_of_numbers_less_than_i(10),
            2640
        );
    }
}
