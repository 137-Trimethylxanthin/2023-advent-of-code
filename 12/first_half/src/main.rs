use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn startgroup(record: &Vec<char>, check: &Vec<usize>, i: usize, j: usize) -> usize {
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

    return nbways(record, check, i + check[j] + 1, j + 1);
}

fn nbways(record: &Vec<char>, check: &Vec<usize>, i: usize, j: usize) -> usize {
    if i >= record.len() && j >= check.len() {
        return 1;
    } else if i >= record.len() {
        return 0;
    }

    match record[i] {
        '.' => return nbways(record, check, i + 1, j),
        '#' => return startgroup(record, check, i, j),
        '?' => return startgroup(record, check, i, j) + nbways(record, check, i + 1, j),
        _ => (),
    }

    0
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
        let record: Vec<char> = parts[0].chars().collect();
        let check: Vec<usize> = parts[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect();

        s += nbways(&record, &check, 0, 0);
    }

    s
}

fn main() {


    let solution = read_file("src/input.txt");

    println!("Solution : {}", solution);
}