use std::env;
use std::time::{Duration, SystemTime};

fn main() {
    let number = env::args()
        .nth(1)
        .and_then(|x| x.parse::<u64>().ok())
        .unwrap_or(20);
    println!(
        "Searching for the smallest divasible number from 1 to {}",
        number
    );
    let benchmark_start = SystemTime::now();
    println!(
        "The smallest evenly divisable number is {}.",
        find_smallest_divisable_number(number)
    );
    let benchmark_end = SystemTime::now().duration_since(benchmark_start).unwrap();

    let format_number = |duration: Duration| -> String {
        let secs = duration.as_secs();

        if secs >= 60 {
            format!("{}m {:02}s", secs / 60, secs % 60)
        } else {
            format!("{}.{:02}s", secs, duration.subsec_nanos() / 10_000_000)
        }
    };

    println!("Calucation took {}.", format_number(benchmark_end));
}

fn find_smallest_divisable_number(max_number: u64) -> u64 {
    let mut number: u64 = 1;

    loop {
        if !is_divisable_by_anything_until_i(number, max_number) {
            number += 1;
        } else {
            return number;
        }
    }
}

fn is_divisable_by_anything_until_i(number: u64, max_number: u64) -> bool {
    macro_rules! max_number_modulo_check {
        ($number: expr) => {
            if max_number >= $number && number % $number != 0 {
                return false;
            }
        };
    }

    max_number_modulo_check!(5);
    max_number_modulo_check!(3);
    max_number_modulo_check!(2);

    // checking for 1-4 is useless, because they are either checked before this block or wouldn't make sense(1)
    for i in 4..=max_number {
        if number % i != 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_divisable_by_anything_until_i;

    #[test]
    fn test_is_divisable_by_anything_between_1_and_10() {
        assert_eq!(is_divisable_by_anything_until_i(2520, 10), true);
    }
}
