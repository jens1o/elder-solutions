fn main() {
    println!(
        "Largest palindrome number of three digit multiplicants: {}",
        greatest_palindrome_number(3)
    );
}

fn greatest_palindrome_number(max_digits: u32) -> u64 {
    assert!(max_digits > 0);

    let mut i = 0;
    let j = 10_u64.pow(max_digits) - 1;

    let mut greatest_number_found = 0;

    loop {
        if get_digits(i + 1) > max_digits {
            println!("get_digits() abort");
            break;
        }

        i += 1;

        print!("i={}, j={}, ", i, j);

        let product_result = i * j;

        println!("result={}", product_result);
        let product_result_string = format!("{}", product_result);

        if product_result_string.chars().collect::<Vec<_>>()
            == product_result_string.chars().rev().collect::<Vec<_>>()
            && product_result > greatest_number_found
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

}
