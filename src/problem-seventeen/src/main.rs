#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref NUMBER_TO_STRING: HashMap<u16, String> = {
        let mut m = HashMap::with_capacity(25);

        macro_rules! add_entry {
            ($number:expr, $string:expr) => {
                m.insert($number, $string.to_owned());
            };
        }

        add_entry!(0, "zero");
        add_entry!(1, "one");
        add_entry!(2, "two");
        add_entry!(3, "three");
        add_entry!(4, "four");
        add_entry!(5, "five");
        add_entry!(6, "six");
        add_entry!(7, "seven");
        add_entry!(8, "eight");
        add_entry!(9, "nine");
        add_entry!(10, "ten");
        add_entry!(11, "eleven");
        add_entry!(12, "twelve");
        add_entry!(13, "thirteen");
        add_entry!(14, "fourteen");
        add_entry!(15, "fifteen");
        add_entry!(16, "sixteen");
        add_entry!(17, "seventeen");
        add_entry!(18, "eighteen");
        add_entry!(19, "nineteen");
        add_entry!(20, "twenty");
        add_entry!(30, "thirty");
        add_entry!(40, "forty");
        add_entry!(50, "fifty");
        add_entry!(60, "sixty");
        add_entry!(70, "seventy");
        add_entry!(80, "eighty");
        add_entry!(90, "ninety");
        add_entry!(100, "hundred");
        add_entry!(1000, "thousand");

        m
    };
}

fn main() {
    for i in 1..=1000 {
        println!("{}: {}", i, get_name_for_number(i));
    }
}

fn get_name_for_number(number: u16) -> String {
    if number > 1000 {
        panic!("This is only implemented for numbers 0 <= number <= 1000");
    }

    // direct hit!
    if NUMBER_TO_STRING.contains_key(&number) {
        return format!("{}{}", if number >= 100 { "one " } else { "" }, NUMBER_TO_STRING.get(&number).unwrap());
    }

    let mut result_string = String::new();

    let mut min_difference = number;
    let mut closest_number = 0;

    // look for the number we know is the closest number to the we were given
    for (key, _) in NUMBER_TO_STRING.iter().filter(|(key, _)| key < &&number) {
        // calculate difference to value
        let difference = number - key;

        // if we are closer than what we already saw, remember this.
        if difference < min_difference {
            min_difference = difference;
            closest_number = *key;
        }
    }

    // ratio >= 1
    let ratio: u16 = number / closest_number;
    // what is missing yet
    let number_left = number - (ratio * closest_number);

    // put ratio in front of number
    if number >= 100 {
        assert!(NUMBER_TO_STRING.get(&ratio).is_some());

        result_string.push_str(NUMBER_TO_STRING.get(&ratio).unwrap());
        result_string.push_str(&" ".to_owned());
    }

    result_string.push_str(NUMBER_TO_STRING.get(&closest_number).unwrap());

    if number_left > 0 {
        if number > 100 {
            result_string.push_str(&" and ".to_owned());
        } else if number_left < 10 {
            result_string.push_str(&"-".to_owned());
        } else {
            unreachable!();
        }

        // call what is left
        result_string.push_str(&get_name_for_number(number - (ratio * closest_number)));
    }

    result_string
}