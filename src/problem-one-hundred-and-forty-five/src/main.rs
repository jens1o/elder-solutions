use std::collections::HashSet;
use std::env;
use std::time::SystemTime;

fn main() {
    let max_number: u32 = env::args()
        .nth(1)
        .and_then(|x| x.parse().ok())
        .and_then(|x| if x <= 10 { None } else { Some(x) }) // a value of less than 10 is not allowed
        .unwrap_or(1000);

    println!("Looking for reversible numbers below {}…", max_number);

    let benchmark_start = SystemTime::now();

    let mut reversible_numbers: HashSet<u32> = HashSet::with_capacity(200);

    // checking for <= 10 is useless.
    for number in 11..=max_number {
        let number_string = format!("{}", number);

        if number_string.ends_with('0') {
            // leading/trailing zero, mismatch
            // there is no need to check for starts_with, because then the other
            // number would also end with a trailing zero
            continue;
        }

        let reversed_number = number_string
            .chars()
            .rev()
            .collect::<String>() // put the numbers again to a string
            .parse::<u32>() // parse as an integer again so we can do fancy math
            .unwrap();

        let sum = number + reversed_number;

        let result = format!("{}", sum)
            .chars()
            .all(|x| x.to_string().parse::<u32>().unwrap() % 2 != 0);

        if result {
            reversible_numbers.insert(number);
        }
    }

    let benchmark_duration = SystemTime::now().duration_since(benchmark_start).unwrap();
    println!();
    println!("Calculation took {:?}.", benchmark_duration);

    println!(
        "There are {} reversible numbers below {}.",
        reversible_numbers.len(),
        max_number
    );
}
