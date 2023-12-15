use std::fs::{File, read};
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;


// i acidentally deleted the code for part 1, and overwrote it with part 2 so thats why it looks like this
fn main() {
    let path = "src/input.txt";
    let grid = read_grid_from_file(path).unwrap();
    let load = tilt_north_and_calculate_load(grid);
    println!("Load: {}", load);
}



fn read_grid_from_file(path: &str) -> io::Result<Vec<Vec<char>>> {
    let path = Path::new(path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    Ok(grid)
}



fn tilt_north_and_calculate_load(grid: Vec<Vec<char>>) -> i32 {
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
    ans

}