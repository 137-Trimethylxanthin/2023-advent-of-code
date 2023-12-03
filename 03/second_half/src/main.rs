use std::fs::File;
use std::io::{BufReader, self, BufRead};
mod test;

fn main() {
    let blueprint = read_file_to_vec("src/input.txt");
    let numbers = find_parts_numbers(blueprint);
    let sum = sum_of_vec(numbers);
    println!("your sum is: {}", sum);
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

fn check_if_is_symbol(c:char) -> bool{
    if c.is_ascii_punctuation() && c != '.'{
        return true;
    }
    return false;
}

fn find_symbols_around(parts:Vec<Vec<char>>, y:usize, x_begin:usize, x_end:usize) -> bool{
    let mut out:bool = false;

        if x_begin != 0{
            if check_if_is_symbol(parts[y][x_begin-1]){
                out = true;
            }
            if y != 0 {
                if check_if_is_symbol(parts[y-1][x_begin-1]){
                    out = true;
                }

            }
            if y < parts.len() - 1 {
                if check_if_is_symbol(parts[y+1][x_begin-1]){
                    out = true;
                }

            }

        }
        if x_end < parts[y].len() - 1{
            if check_if_is_symbol(parts[y][x_end+1]){
                out = true;
            }
            if y != 0 {
                if check_if_is_symbol(parts[y-1][x_end+1]){
                    out = true;
                }
            }
            if y < parts.len() - 1 {
                if check_if_is_symbol(parts[y+1][x_end+1]){
                    out = true;
                }
            }
        }


    if y != 0 {
        for i in 0..(x_end+1 - x_begin ) {
            if check_if_is_symbol(parts[y-1][x_begin + i ]){
                out = true;
            }
        }
    }
    if y < parts.len() - 1 {
        for i in 0..(x_end+1 - x_begin ){
            if check_if_is_symbol(parts[y+1][x_begin+i]){
                out = true;
            }
        }
    }

    return out
}

fn get_number_from_vec(parts:Vec<Vec<char>>, y:usize, x_begin:usize, x_end:usize) -> i32{
    let mut numbers:Vec<char> = Vec::new();
    let mut number = 0;
    for i in x_begin..x_end {
        numbers.push(parts[y][i]);
    }
    for (i,c) in numbers.iter().enumerate() {
        number += numbers[(numbers.len()-1)-i].to_digit(10).unwrap() as i32 * 10_i32.pow(i as u32)
    }
    return number;
}

fn find_parts_numbers(parts:Vec<Vec<char>>) -> Vec<i32>{
    let mut is_a_number = false;
    let mut x_begin = 0;
    let mut x_end = 0;
    let mut y = 0;
    let mut real_parts:Vec<i32> = Vec::new();

    for (i,part) in parts.iter().enumerate() {

        for (j,p) in part.iter().enumerate() {
            if p.is_numeric() && !is_a_number{
                is_a_number = true;
                x_begin = j;
                x_end = j;
                y = i;
            }
            if is_a_number {

                if p.is_numeric() && y == i {
                    x_end += 1;
                } else {
                    is_a_number = false;
                    if find_symbols_around(parts.clone(), y,x_begin, x_end-1){
                        real_parts.push(get_number_from_vec(parts.clone(), y,x_begin, x_end))
                    }
                }
            } else {

            }
        }
        if is_a_number {
            is_a_number = false;
            if find_symbols_around(parts.clone(), y,x_begin, x_end-1){
                real_parts.push(get_number_from_vec(parts.clone(), y,x_begin, x_end))
            }
        }
    }
    return real_parts;
}

fn sum_of_vec(numbers:Vec<i32>) -> i32{
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    return sum;
}

