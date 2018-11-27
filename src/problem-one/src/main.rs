use std::collections::HashSet;
use std::env;
use std::time::SystemTime;

fn main() {
    let max_number = env::args()
        .nth(1)
        .and_then(|x| x.parse::<i64>().ok())
        .unwrap_or(1000);

    let mut multipliers = env::args()
        .skip(2) // Skip max number entry
        .map(|x| x.parse::<i64>().expect("Invalid number given."))
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
    let products = get_products_smaller_max(&mut multipliers, &max_number);
    println!("Products < Max number: {:?}", products);

    println!(
        "Sum: {} (took {:?})",
        get_sum(&mut products.into_iter()),
        // unwrap is safe because code-wise `SystemTime::now` is later than `benchmark_start`.
        SystemTime::now().duration_since(benchmark_start).unwrap()
    );
}

fn get_products_smaller_max(numbers: &mut Iterator<Item = i64>, max_number: &i64) -> Vec<i64> {
    let mut results = vec![];

    for number in numbers {
        let mut i = 1;

        while number * i < *max_number {
            results.push(number * i);
            i += 1;
        }
    }

    results
}

fn get_sum(numbers: &mut Iterator<Item = i64>) -> i64 {
    numbers.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_products() {
        let mut products = get_products_smaller_max(&mut vec![3, 5].into_iter(), &10);

        products.sort();
        assert_eq!(products, vec![3, 5, 6, 9]);
    }

    #[test]
    fn test_get_sum() {
        assert_eq!(get_sum(&mut vec![3, 5, 6, 9].into_iter()), 23);
    }
}
