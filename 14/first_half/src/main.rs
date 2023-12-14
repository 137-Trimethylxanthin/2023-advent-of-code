use std::fs::{File, read};
use std::io::{BufRead, BufReader, Read};

fn main() {
    let path = "src/test.txt";
    let sum = calculate_total_load(path);
    println!("Sum: {}", sum);
}


fn calculate_total_load(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let cycles = 1_000_000_000;

    let mut grid: Vec<Vec<char>> = reader.lines()
        .filter_map(Result::ok)
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();


    let mut sofar = std::collections::HashMap::new();
    let mut steps = std::collections::HashMap::new();
    steps.insert(0, grid.clone());
    sofar.insert(grid.clone(), 0);
    let mut ans = None;

    for i in 0..cycles {
        grid = cycle_grid(grid);
        if sofar.contains_key(&grid) {
            ans = Some((i + 1, sofar[&grid]));
            break;
        }
        steps.insert(i + 1, grid.clone());
        sofar.insert(grid.clone(), i + 1);
    }

    if let Some((i, j)) = ans {
        let key = ((cycles - j) % (i - j)) + j;
        if let Some(grid) = steps.get(&key) {
            return finde_me_O(grid);
        }
    }
    0
}

fn rotate_grid(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut ngrid = vec![vec!['.'; grid.len()]; grid[0].len()];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            ngrid[j][i] = grid[i][j];
        }
    }
    for row in &mut ngrid {
        row.reverse();
    }
    ngrid
}

fn transform_grid(mut grid: Vec<Vec<char>>, rots: usize) -> (i32, Vec<Vec<char>>) {
    let rots = rots % 4;
    for _ in 0..rots {
        grid = rotate_grid(grid);
    }

    let mut ngrid = vec![vec!['.'; grid[0].len()]; grid.len()];
    let mut ans = 0;
    for j in 0..grid[0].len() {
        let mut os_in_queue = 0;
        for i in (0..grid.len()).rev() {
            if grid[i][j] == '#' {
                for k in 0..os_in_queue {
                    ans += grid.len() as i32 - (i as i32 + 1 + k as i32);
                    ngrid[i + 1 + k][j] = 'O';
                }
                os_in_queue = 0;
                ngrid[i][j] = '#';
            } else if grid[i][j] == 'O' {
                os_in_queue += 1;
            }
        }
        for k in 0..os_in_queue {
            ans += grid.len() as i32 - k as i32;
            ngrid[k][j] = 'O';
        }
    }

    for _ in 0..4 - rots {
        ngrid = rotate_grid(ngrid);
    }

    (ans, ngrid)
}

fn cycle_grid(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 0..4 {
        grid = transform_grid(grid, i).1;
    }
    grid
}

fn finde_me_O(grid: &[Vec<char>]) -> i32 {
    let mut ans = 0;
    for (i, row) in grid.iter().enumerate() {
        for &ch in row {
            if ch == 'O' {
                ans += grid.len() as i32 - i as i32;
            }
        }
    }
    ans
}


fn spin_cycle(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)]; // north, west, south, east

    for &(dx, dy) in &directions {
        let mut new_grid = grid.clone();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 'O' {
                    let mut x = i as i32;
                    let mut y = j as i32;
                    while (0..grid.len() as i32).contains(&(x + dx)) && (0..grid[0].len() as i32).contains(&(y + dy)) && grid[x as usize + dx as usize][y as usize + dy as usize] != '#' {
                        x += dx;
                        y += dy;
                    }
                    new_grid[x as usize][y as usize] = 'O';
                    if x != i as i32 || y != j as i32 {
                        new_grid[i][j] = '.';
                    }
                }
            }
        }
        grid = new_grid;
    }

    grid
}