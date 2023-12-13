use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn score(g: &Vec<Vec<char>>) -> i32 {
    for y in 1..g.len() {
        if g[0..y].iter().rev().zip(g[y..].iter()).all(|(a, b)| a == b) {
            return (y as i32) * 100;
        }
    }

    let g: Vec<Vec<char>> = (0..g[0].len()).map(|i| g.iter().map(|x| x[i]).collect()).collect();

    for y in 1..g.len() {
        if g[0..y].iter().rev().zip(g[y..].iter()).all(|(a, b)| a == b) {
            return y as i32;
        }
    }

    0
}

fn main() {
    let file = File::open("src/input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let mut grids: Vec<Vec<Vec<char>>> = vec![vec![]];
    for line in lines {
        if line.len() == 0 {
            grids.push(vec![]);
        } else {
            grids.last_mut().unwrap().push(line.chars().collect());
        }
    }

    let result: i32 = grids.iter().map(|g| score(g)).sum();
    println!("{}", result);
}