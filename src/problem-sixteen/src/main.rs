use std::env;

fn main() {
    let exponent = env::args()
        .nth(1)
        .and_then(|x| x.parse::<u32>().ok())
        .unwrap_or(15);

    println!(
        "The result of the sum of the digits of the number 2 ** {} is {}",
        exponent,
        get_digit_sum_of_exponent(exponent)
    );
}

fn get_digit_sum_of_exponent(exponent: u32) -> u64 {
    let sum = 2u64.pow(exponent);
    let sum_string = sum.to_string();

    let mut result = 0;

    for digit in sum_string.chars() {
        result += digit.to_string().parse::<u64>().unwrap();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::get_digit_sum_of_exponent;

    #[test]
    fn get_digit_sum() {
        assert_eq!(get_digit_sum_of_exponent(15), 26);
    }
}
