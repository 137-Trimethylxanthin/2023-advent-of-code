use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    let mut sum:u32 = 0;
    let vec_num = read_file_to_vec("src/input.txt");
    for number in vec_num {
        sum += Holiday_ASCII_String_Helper_algorithm(&number);
    }

    println!("Hash: {}", sum);
}

fn read_file_to_vec(filename: &str) -> Vec<String> {

    let mut file = File::open(filename).expect("File not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not read file");


    content.split(",").map(|s| s.to_string()).collect()
}


fn Holiday_ASCII_String_Helper_algorithm(string: &str) -> u32 {
    let mut result = 0;
    for c in string.chars() {
        result += c as u32;
        result *= 17;
        result %= 256;
    }

    result

}