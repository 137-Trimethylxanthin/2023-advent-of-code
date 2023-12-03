use std::collections::hash_map::VacantEntry;
use std::fs::File;
use std::io::{BufReader, self, BufRead};
mod test;

fn main() {
    let blueprint = read_file_to_vec("src/input.txt");
    let sum = combine_numbers(blueprint);
    let gear_number = get_gear_numbers_of_a_vec(sum);
    println!("your gear number is: {:?}", gear_number);
}

fn read_file_to_vec(path:&str) -> Vec<Vec<char>>{
    let file:File = File::open(path).unwrap();
    let mut parts:Vec<Vec<char>> = Vec::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut inner_vec:Vec<char> = Vec::new();
        for char in line.unwrap().chars() {
            inner_vec.push(char);
        }
        parts.push(inner_vec);
    }
    return parts;
}

fn combine_numbers(parts: Vec<Vec<char>>) -> Vec<Vec<String>> {
    let mut new_parts: Vec<Vec<String>> = Vec::new();

    for part in parts {
        let mut new_part: Vec<String> = Vec::new();
        let mut number = String::new();

        for &char in &part {
            if char.is_digit(10) {
                number.push(char);
            } else {
                if !number.is_empty() {
                    for _ in number.chars() {
                        new_part.push(number.clone());
                    }
                    number = String::new();
                }
                new_part.push(char.to_string());
            }
        }

        if !number.is_empty() {
            for _ in number.chars() {
                new_part.push(number.clone());
            }
        }

        new_parts.push(new_part);
    }

    new_parts
}

fn get_gear_numbers_of_a_vec(list:Vec<Vec<String>>) -> i32{
    let mut int = 0;

    for (x, row) in list.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if list[x][y] == "*" {
                int += get_gear_num(list.clone(), x, y);
            }
        }
    }

    return int;
}

fn get_gear_num(list:Vec<Vec<String>>, x:usize, y:usize) -> i32 {
    let mut unique_numbers = Vec::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];

    for (dx, dy) in &directions {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        if new_x >= 0 && new_y >= 0 && new_x < list.len() as i32 && new_y < list[0].len() as i32 {
            if let Ok(num) = list[new_x as usize][new_y as usize].parse::<i32>() {
                if !unique_numbers.contains(&num) {
                    unique_numbers.push(num);
                }
            }
        }
    }

    if unique_numbers.len() == 2 {
        unique_numbers[0] * unique_numbers[1]
    } else {
        0
    }
}