use std::collections::HashSet;
use std::env;
use std::time::SystemTime;

fn main() {
    let max_number: u128 = env::args()
        .nth(1)
        .and_then(|x| x.parse::<_>().ok())
        .and_then(|x| if x <= 10 { None } else { Some(x) })
        .unwrap_or(10_000);

    println!("Generating lychrel numbers below {}.", max_number);

    let benchmark_start = SystemTime::now();

    let lychrel_numbers = get_lychrel_numbers(max_number);

    let benchmark_duration = SystemTime::now().duration_since(benchmark_start).unwrap();

    println!(
        "There are {} lychrel numbers below {} (took {:?}).",
        lychrel_numbers.len(),
        max_number,
        benchmark_duration
    )y
}

#[inline(always)]
fn is_palindromic(number_string: String) -> bool {
    number_string == number_string.chars().rev().collect::<String>()
}

#[inline(always)]
fn reverse_number(number_string: String) -> u128 {
    number_string
        .chars()
        .rev()
        .collect::<String>()
        .parse::<_>()
        .unwrap()
}

fn get_lychrel_numbers(max: u128) -> HashSet<u128> {
    assert!(max > 10);

    let mut results = HashSet::with_capacity(max as usize / 3);

    'outer: for i in 10..=max {
        let mut current_iteration: u8 = 1;
        let mut current_number: u128 = i;

        while current_iteration < 50 {
            current_iteration += 1;

            let number_string = current_number.to_string();

            let reversed_current_number = reverse_number(number_string);

            let current_sum = current_number
                .checked_add(reversed_current_number)
                .expect("Current sum overflowed.");

            let current_sum_string = current_sum.to_string();

            if is_palindromic(current_sum_string) {
                results.insert(current_number);
                continue 'outer;
            } else {
                current_number = current_sum;
            }
        }
        // hit 50 iterations with the current
    }

    results
}
