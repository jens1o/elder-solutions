#![deny(missing_docs)]

//! # Problem one
//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

use std::collections::HashSet;
use std::env;
use std::time::SystemTime;

fn main() {
    let max_number = env::args()
        .nth(1)
        .and_then(|x| x.parse::<i64>().ok())
        .unwrap_or(1000); // fallback to default value of exercise if no/malformed input provided

    let mut multipliers = env::args()
        .skip(2) // Skip max number entry
        .map(|x| x.parse::<i64>().expect("Invalid number given.")) // Iff numbers are given, expect valid ones.
        .collect::<HashSet<_>>();

    if multipliers.is_empty() {
        // push some default values
        multipliers.insert(3);
        multipliers.insert(5);
    }

    let mut multipliers = multipliers.into_iter();

    println!("Max number: {}", &max_number);
    println!("Multipliers: {:?}", &multipliers);

    let benchmark_start = SystemTime::now();
    let products = get_products_smaller_max(&mut multipliers, max_number);

    // leaving this - commented out - saves probably 50µs of benchmark time. :D
    // println!("Products < Max number: {:?}", products);

    println!(
        "Sum: {} (took {:?})",
        get_sum(&mut products.into_iter()),
        // unwrap is safe because code-wise `SystemTime::now` is later than `benchmark_start`.
        SystemTime::now().duration_since(benchmark_start).unwrap()
    );
}

/// Returns all products of the Iterator entry that are less than max_number.
///
/// # Example:
/// ```rust
/// let mut products = get_products_smaller_max(&mut vec![3, 5].into_iter(), &10);
/// products.sort();
/// assert_eq!(products, vec![3, 5, 6, 9]);
/// ```
/// # Note:
/// They are not sorted by default. A number may appear multiple times.
pub fn get_products_smaller_max(
    numbers: &mut Iterator<Item = i64>,
    max_number: i64,
) -> HashSet<i64> {
    let mut results: HashSet<i64> = HashSet::with_capacity(max_number as usize);

    for number in numbers {
        let mut i = 1;

        // go from bottom to top and check whether we are still less than
        // `max_number`.
        while number * i < max_number {
            results.insert(number * i);
            i += 1;
        }
    }

    results
}

/// Returns the sum for all numbers in this iterator.
/// # Example:
/// ```rust
/// assert_eq!(get_sum(&mut vec![3, 5, 6, 9].into_iter()), 23);
/// ```
#[inline(always)]
pub fn get_sum(numbers: &mut Iterator<Item = i64>) -> i64 {
    numbers.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_products() {
        let mut products = get_products_smaller_max(&mut vec![3, 5].into_iter(), 10)
            .iter()
            .map(|x| *x)
            .collect::<Vec<_>>();

        products.sort();
        assert_eq!(products, vec![3, 5, 6, 9]);
    }

    #[test]
    fn test_get_sum() {
        assert_eq!(get_sum(&mut vec![3, 5, 6, 9].into_iter()), 23);
    }
}
