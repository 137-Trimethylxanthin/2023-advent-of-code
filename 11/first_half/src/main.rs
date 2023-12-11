use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let univers = read_univers_to_vec("src/input.txt");

    let mut univers = expand(univers);


    let mut galaxies = vec![];

    for i in 0..univers.len() {
        for j in 0..univers[0].len() {
            if univers[i][j] == '#' {
                galaxies.push((i, j));
                univers[i][j] = '.';
            }
        }
    }

    let n = galaxies.len();

    let mut unique_distances = vec![];
    for i in 0..n {
        for j in i+1..n {
            let d = shortest_path(&univers, galaxies[i], galaxies[j]);
            unique_distances.push(d);
        }
    }



    let sum: usize = unique_distances.iter().sum::<usize>();
    println!("sum is: {}", sum);
}

fn shortest_path(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    visited[start.0][start.1] = true;

    while let Some(((x, y), d)) = queue.pop_front() {
        if (x, y) == end {
            return d;
        }

        for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < grid.len() as i32 && ny < grid[0].len() as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if !visited[nx][ny] && grid[nx][ny] != '#' {
                    visited[nx][ny] = true;
                    queue.push_back(((nx, ny), d + 1));
                }
            }
        }
    }

    usize::MAX
}




fn expand(univers: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded = univers.clone();
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

    for &row in empty_rows.iter().rev() {
        expanded.insert(row, vec!['.'; univers[0].len()]);
    }

    for &col in empty_cols.iter().rev() {
        for row in expanded.iter_mut() {
            row.insert(col, '.');
        }
    }

    expanded
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