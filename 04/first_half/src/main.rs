use std::fs::File;
use std::io::{BufRead, BufReader};

mod test;

fn main() {
    let sum = solve_file("src/input.txt");
    println!("your final sum: {}", sum);

}


fn solve_file(file: &str) -> i32 {
    let mut sum = 0;
    let mut file = File::open(file).expect("File not found");

    let mut reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut scratch = 0.5;
        let part:Vec<&str> = line.split(":").collect();
        let ticket = part[1];
        let part:Vec<&str> = ticket.split("|").collect();
        let winning_num = part[0];
        let my_num = part[1];
        let winning_list:Vec<i32> = get_numbers_from_string(winning_num);
        let my_list:Vec<i32> =  get_numbers_from_string(my_num);

        for win in winning_list {
            if my_list.contains(&win){
                scratch *= 2.0;
            }
        }
        sum += scratch as i32;
    }


    return sum;
}


fn get_numbers_from_string(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect()
}