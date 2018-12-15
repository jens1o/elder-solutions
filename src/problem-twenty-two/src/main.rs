use std::fs;
use std::time::SystemTime;

fn main() {
    let filename = "p022_names.txt";

    let names = fs::read_to_string(filename).expect("Cannot read file.");

    let benchmark_start = SystemTime::now();

    // remove first and last character because they are just quote-marks
    let (_, names) = names.split_at(1);
    let (names, _) = names.split_at(names.len() - 1);

    let mut names: Vec<&str> = names.split("\",\"").collect::<Vec<_>>();

    // sort them alphabetically
    names.sort();

    let mut score_sum = 0;

    for (i, name) in names.iter().enumerate() {
        let score = name
            .chars()
            .map(|x| x as usize)
            .map(|x| x - 64) // get numeric values (A = 1, code(A) = 65)
            .sum::<usize>()
            * (i + 1);

        score_sum += score;
    }

    let benchmark_end = SystemTime::now().duration_since(benchmark_start).unwrap();

    println!("Score of file: {}", score_sum);
    println!("Calculation took: {:?}", benchmark_end);
}
