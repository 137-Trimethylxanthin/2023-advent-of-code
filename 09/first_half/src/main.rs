use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "src/input.txt";
    let result = predict_the_future(path);
    println!("Result: {}", result);
}

fn predict_the_future(path: &str) -> i32 {
    let file = File::open(path).expect("File not found");
    let reader = BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let mut current = vec![numbers];

        while !all_zero(current.last().unwrap()) {
            let differences = differences(current.last().unwrap());
            current.push(differences);
        }

        current.last_mut().unwrap().push(0);

        for i in (1..current.len()).rev() {
            let index =  current[i].len() - 1;
            let extrapolated = current[i - 1][index] + current[i][index];
            current[i - 1].push(extrapolated);
        }

        total += current[0].last().unwrap().clone();
    }

    total
}

fn all_zero(numbers: &[i32]) -> bool {
    numbers.iter().all(|&n| n == 0)
}

fn differences(numbers: &[i32]) -> Vec<i32> {
    numbers.windows(2).map(|w| w[1] - w[0]).collect()
}