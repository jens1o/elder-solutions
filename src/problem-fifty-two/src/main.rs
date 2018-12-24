#[cfg(feature = "stats")]
use std::collections::HashMap;

use std::collections::HashSet;
use std::time::SystemTime;

const MAX_MULTIPLIER: u32 = 6;

fn main() {
    let benchmark_start = SystemTime::now();

    let mut smallest_number: u32 = 0;

    #[cfg(feature = "stats")]
    let mut multiplier_abort_map: HashMap<u32, u32> = HashMap::with_capacity(100);

    'outest: for n in 1..u32::max_value() {
        let mut digits: HashSet<char> = HashSet::new();
        let mut multiplier: u32 = 1;

        'inner: while multiplier != MAX_MULTIPLIER {
            multiplier += 1;

            let result = n * multiplier;

            let result_digits = result.to_string().chars().collect::<HashSet<_>>();

            // in the first step, the hashset is empty, so we need to fill it first.
            if digits.is_empty() {
                for digit in result_digits {
                    digits.insert(digit);
                }
                continue;
            }

            for digit in result_digits {
                if !digits.contains(&digit) {
                    // try next number, because we found a number that is not in the other one.

                    #[cfg(feature = "stats")]
                    {
                        multiplier_abort_map
                            .entry(multiplier)
                            .and_modify(|x| *x += 1)
                            .or_insert(1);
                    }

                    continue 'outest;
                }
            }
        }
        smallest_number = n;
        break;
    }

    let benchmark_duration = SystemTime::now().duration_since(benchmark_start).unwrap();

    println!(
        "The smallest number that is permuted is {}. Took {:?}",
        smallest_number, benchmark_duration
    );

    #[cfg(feature = "stats")]
    {
        println!("Abort map statistics: {:?}", multiplier_abort_map);
    }
}
