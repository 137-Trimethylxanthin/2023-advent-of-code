use std::fs::File;
use std::io::{BufRead, Read};
use regex::Regex;
use std::str::FromStr;
mod test;

fn main() {
    let u = race_simulation("src/input.txt");
    println!("you won: {}", u);
}

fn race_simulation(path:&str) -> u32 {
    let mut time_to_distance:Vec<Vec<u32>>= Vec::new();
    let mut file:File = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let parts:Vec<&str> = buf.split("\n").collect();
    let time_records = extract_numbers(parts[0]);
    let distance_records = extract_numbers(parts[1]);



    println!("{:?}", time_records);
    println!("{:?}", distance_records);

    let mut total_wins = 1;

    for i in 0..time_records.len() {
        let mut ways_to_win = 0;
        let time = time_records[i];
        let record_distance = distance_records[i];

        for button_time in 0..time {
            let distance = button_time * (time - button_time);

            if distance > record_distance {
                ways_to_win += 1;
            }
        }

        total_wins *= ways_to_win;
    }


    return total_wins;



}


fn extract_numbers(s: &str) -> Vec<u32> {
    let re = Regex::new(r"\d+").unwrap();
    re.captures_iter(s)
        .map(|cap| u32::from_str(&cap[0]).unwrap())
        .collect()
}

