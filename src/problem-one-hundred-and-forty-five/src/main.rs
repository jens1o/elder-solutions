extern crate num_cpus;

use std::cmp;
use std::env;
use std::io;
use std::sync::mpsc::channel;
use std::thread;
use std::time::SystemTime;

/// calculating below this number is considered useless
const START_NUMBER: u32 = 10;

fn main() -> Result<(), io::Error> {
    let max_number: u32 = env::args()
        .nth(1)
        .and_then(|x| x.parse().ok())
        .and_then(|x| if x <= 10 { None } else { Some(x) }) // a value of less than 10 is not allowed
        .unwrap_or(1000);

    println!("Looking for reversible numbers below {}…", max_number);

    let number_of_threads = cmp::max(num_cpus::get(), 1);

    println!("Launching {} threads.", number_of_threads);

    let mut reversible_numbers: u32 = 0;
    let benchmark_start = SystemTime::now();

    let (tx, rx) = channel();

    let reversible_numbers_to_calculate_per_thread: usize =
        ((max_number - START_NUMBER) / (number_of_threads as u32)) as usize;

    println!(
        "Every thread needs to calculate ~{} numbers.",
        reversible_numbers_to_calculate_per_thread
    );

    for thread_number in 1..=number_of_threads {
        let start_position = cmp::max(
            (thread_number - 1) * reversible_numbers_to_calculate_per_thread,
            START_NUMBER as usize,
        );

        let sender = tx.clone();

        thread::spawn(move || {
            println!("Thread {} started.", thread_number);

            let sub_results = find_reversible_number_below(
                start_position as u32,
                // if this is the last thread, calculate everything to the top, otherwise we would miss some because of rounding errors
                if thread_number == number_of_threads {
                    max_number
                } else {
                    (start_position + reversible_numbers_to_calculate_per_thread) as u32
                },
            );

            sender
                .send(sub_results)
                .expect("Cannot send sub-results to main thread!");

            println!("Thread {} done!", thread_number);
        });
    }

    // gather result from threads
    for _ in 1..=number_of_threads {
        for sub_result in rx.recv() {
            reversible_numbers += sub_result;
        }
    }

    let benchmark_duration = SystemTime::now().duration_since(benchmark_start).unwrap();
    println!();
    println!("Calculation took {:?}.", benchmark_duration);

    println!(
        "There are {} reversible numbers below {}.",
        reversible_numbers, max_number,
    );

    Ok(())
}

fn find_reversible_number_below(start_number: u32, max_number: u32) -> u32 {
    let mut reversible_numbers_found: u32 = 0;

    debug_assert!(start_number < max_number);

    for number in start_number..=max_number {
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

        if format!("{}", sum)
            .chars()
            .all(|x| x.to_string().parse::<u8>().unwrap() % 2 != 0)
        {
            reversible_numbers_found += 1
        }
    }

    reversible_numbers_found
}
