#![feature(integer_atomics)]

extern crate num_cpus;

use std::cmp;
use std::collections::HashSet;
use std::env;
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime};

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

    let benchmark_start = SystemTime::now();
    let mut thread_handles: Vec<JoinHandle<_>> = Vec::with_capacity(number_of_threads);
    let reversible_numbers: Arc<Mutex<Vec<u32>>> =
        Arc::new(Mutex::new(Vec::with_capacity(200 * number_of_threads)));

    let finished_threads = Arc::new(Mutex::new(AtomicUsize::new(0)));

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

        let current_end = start_position + reversible_numbers_to_calculate_per_thread;

        let reversible_numbers = reversible_numbers.clone();
        let finished_threads = finished_threads.clone();

        thread_handles.push(thread::spawn(move || {
            println!("Thread {} started.", thread_number);

            let sub_results = find_reversible_number_below(
                start_position as u32,
                // if this is the last thread, calculate everything to the top, otherwise we would miss some because of rounding errors
                if thread_number == number_of_threads {
                    max_number
                } else {
                    current_end as u32
                },
            );

            reversible_numbers.lock().unwrap().extend(sub_results);

            finished_threads
                .lock()
                .unwrap()
                .fetch_add(1, Ordering::SeqCst);
        }));
    }

    loop {
        thread::sleep(Duration::from_nanos(250));
        let finished_threads = finished_threads.lock().unwrap().load(Ordering::SeqCst);
        if finished_threads == number_of_threads {
            break;
        }
    }

    let benchmark_duration = SystemTime::now().duration_since(benchmark_start).unwrap();
    println!();
    println!("Calculation took {:?}.", benchmark_duration);

    let reversible_numbers = reversible_numbers.lock().unwrap();

    let mut reversible_numbers = reversible_numbers.iter().collect::<Vec<_>>();
    reversible_numbers.sort();

    println!(
        "There are {} reversible numbers below {}.",
        reversible_numbers.len(),
        max_number,
    );

    Ok(())
}

fn find_reversible_number_below(start_number: u32, max_number: u32) -> HashSet<u32> {
    let mut reversible_numbers: HashSet<u32> = HashSet::with_capacity(200);

    assert!(start_number < max_number);

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

        let result = format!("{}", sum)
            .chars()
            .all(|x| x.to_string().parse::<u32>().unwrap() % 2 != 0);

        if result {
            reversible_numbers.insert(number);
        }
    }

    reversible_numbers
}
