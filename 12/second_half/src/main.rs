use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn startgroup(record: &Vec<char>, check: &Vec<usize>, i: usize, j: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(&result) = memo.get(&(i, j)) {
        return result;
    }

    if j >= check.len() {
        return 0;
    }

    for k in 0..check[j] {
        if i + k >= record.len() {
            return 0;
        }

        match record[i + k] {
            '#' | '?' => continue,
            '.' => return 0,
            _ => (),
        }
    }

    if i + check[j] < record.len() && record[i + check[j]] == '#' {
        return 0;
    }

    let result = nbways(record, check, i + check[j] + 1, j + 1, memo);
    memo.insert((i, j), result);
    result
}

fn nbways(record: &Vec<char>, check: &Vec<usize>, i: usize, j: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {

    if let Some(&result) = memo.get(&(i, j)) {
        return result;
    }

    if i >= record.len() && j >= check.len() {
        return 1;
    } else if i >= record.len() {
        return 0;
    }

    let result = match record[i] {
        '.' => nbways(record, check, i + 1, j, memo),
        '#' => startgroup(record, check, i, j, memo),
        '?' => startgroup(record, check, i, j, memo) + nbways(record, check, i + 1, j, memo),
        _ => 0,
    };

    memo.insert((i, j), result);
    result
}

fn read_file(path: &str) -> usize {
    let mut input = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ip) = line {
                input.push(ip);
            }
        }
    }
    let mut s = 0;
    for line in input {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut record: Vec<char> = parts[0].chars().collect();
        let mut check: Vec<usize> = parts[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect();


        let mut unfolded_record = Vec::new();
        for _ in 0..5 {
            unfolded_record.extend_from_slice(&record);
            unfolded_record.push('?');
        }
        unfolded_record.pop();

        let mut unfolded_check = Vec::new();
        for _ in 0..5 {
            unfolded_check.extend_from_slice(&check);
        }

        record = unfolded_record;
        check = unfolded_check;


        let mut memo = HashMap::new();

        let value = nbways(&record, &check, 0, 0, &mut memo);
        s += value;
    }

    s
}

fn main() {


    let solution = read_file("src/input.txt");

    println!("Solution : {}", solution);
}