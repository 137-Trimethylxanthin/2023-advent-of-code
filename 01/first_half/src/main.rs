use std::io::{self, BufRead};
use std::fs::File;
mod tests;




fn main() {
    let word_list = read_file_to_vec().expect("wrong");
    println!("Your calibration values are: {}", decode_list(word_list))

}

fn read_file_to_vec() -> io::Result<Vec<String>> {
    let mut word_list:Vec<String> = Vec::new();
    let file = File::open("src/input.txt")?;

    let reader = io::BufReader::new(file);

    for line in reader.lines(){
        word_list.push(line?);
    }

    Ok(word_list)
}

fn decode_line(word: &str) -> i32{

    let left_i: char =   word.chars().nth(word.find(char::is_numeric).unwrap()).unwrap();
    let right_i: char =   reverse_string(word).chars().nth(reverse_string(word).find(char::is_numeric).unwrap()).unwrap();

    let number = format!("{}{}", left_i, right_i);

    string_to_i32(&number).expect("Not a Number you ...")

}


fn decode_list(word_list:Vec<String>) -> i32 {
    let mut sum = 0;
    for word in word_list.iter() {
        sum += decode_line(word);
    }
    sum
}

fn reverse_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.reverse();
    String::from_iter(chars)
}

fn string_to_i32(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

