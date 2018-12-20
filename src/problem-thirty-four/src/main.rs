use std::time::SystemTime;

const MAX_LIMIT: u32 = 1_000_000;

fn main() {
    let benchmark_start = SystemTime::now();

    let mut sum: u32 = 0;

    for x in 3..=MAX_LIMIT {
        if is_curious_number(x) {
            sum += x;
        }
    }

    let benchmark_duration = SystemTime::now().duration_since(benchmark_start).unwrap();

    println!("Sum: {} (took {:?})", sum, benchmark_duration);
}

fn factorial(number: u32) -> u32 {
    match number {
        0 => 1,
        n => n * factorial(n - 1),
    }
}

fn is_curious_number(number: u32) -> bool {
    let digits = format!("{}", number);

    let factorial_digits = digits
        .chars()
        .map(|x| factorial(x.to_string().parse::<u32>().unwrap()));

    factorial_digits.sum::<u32>() == number
}

#[cfg(test)]
mod tests {
    use super::is_curious_number;

    #[test]
    fn test_digit_factorial() {
        assert!(is_curious_number(145));
    }
}
