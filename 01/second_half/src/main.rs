use std::fs::File;
use std::io;
use std::io::BufRead;

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
        word_list.push(replace_numeric_words_with_ints(line?));
    }

    Ok(word_list)
}

fn decode_line(word: &String) -> i32{

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

fn replace_numeric_words_with_ints(mut s: String) -> String {
    let words_to_numbers = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];


    for (word, number) in &words_to_numbers {
        let replacement = format!("{}{}{}", word ,number,word);
        s = s.replace(word, &replacement);
    }
    s
}
