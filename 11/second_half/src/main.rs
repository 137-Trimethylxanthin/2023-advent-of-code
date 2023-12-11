use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let univers = read_univers_to_vec("src/input.txt");

    let (empty_rows, empty_cols) = expand(&univers);

    let mut galaxies = vec![];

    for i in 0..univers.len() {
        for j in 0..univers[0].len() {
            if univers[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    let unique_distances = shortest_path(&galaxies, &empty_rows, &empty_cols, 1_000_000);

    let sum: usize = unique_distances.iter().sum::<usize>();
    println!("sum is: {}", sum);
}


fn shortest_path(galaxies: &Vec<(usize, usize)>, empty_rows: &Vec<usize>, empty_cols: &Vec<usize>, ratio: usize) -> Vec<usize> {
    let mut unique_distances = vec![];
    let n = galaxies.len();

    for i in 0..n {
        for j in i+1..n {
            let g1 = galaxies[i];
            let g2 = galaxies[j];
            let d = (g1.0 as isize - g2.0 as isize).abs() as usize + (g1.1 as isize - g2.1 as isize).abs() as usize;
            let eid = empty_rows.iter().filter(|&&ei| between(ei, g1.0, g2.0)).count();
            let ejd = empty_cols.iter().filter(|&&ej| between(ej, g1.1, g2.1)).count();
            let d = d + (ratio-1)*eid + (ratio-1)*ejd;
            unique_distances.push(d);
        }
    }

    unique_distances
}




fn between(value: usize, bound1: usize, bound2: usize) -> bool {
    let min_bound = bound1.min(bound2);
    let max_bound = bound1.max(bound2);
    value > min_bound && value < max_bound
}

fn expand(univers: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows = vec![];
    let mut empty_cols = vec![];

    for i in 0..univers.len() {
        if !univers[i].contains(&'#') {
            empty_rows.push(i);
        }
    }

    for j in 0..univers[0].len() {
        if univers.iter().all(|row| row[j] != '#') {
            empty_cols.push(j);
        }
    }

    (empty_rows, empty_cols)
}


fn read_univers_to_vec(path:&str) -> Vec<Vec<char>> {
    let mut univers = Vec::new();
    let file = File::open(path).expect("file not found");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        univers.push(line.chars().collect());
    }

    univers
}