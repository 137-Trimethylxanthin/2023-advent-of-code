use std::collections::HashSet;
use std::fs;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn read_grid(inputs: &str) -> (HashSet<Point>, i32, i32, Point) {
    let mut grid = HashSet::new();
    let mut starting_point = Point { x: 0, y: 0 };
    let mut size_x = 0;
    let mut size_y = 0;

    for (y, line) in inputs.lines().enumerate() {
        size_x = line.len() as i32;
        for (x, c) in line.chars().enumerate() {
            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            if c == 'S' {
                starting_point = point;
            } else if c == '.' {
                grid.insert(point);
            }
        }
        size_y = y as i32 + 1;
    }

    (grid, size_x, size_y, starting_point)
}

fn work_p1(inputs: &str, steps: usize) -> usize {
    let (mut grid, _, _, s) = read_grid(inputs);
    let mut q = HashSet::new();
    q.insert(s);

    for _ in 0..steps {
        let mut nq = HashSet::new();
        for p in &q {
            for d in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                let np = Point {
                    x: p.x + d.0,
                    y: p.y + d.1,
                };
                if grid.contains(&np) {
                    nq.insert(np);
                }
            }
        }
        q = nq;
    }

    q.len()
}

fn main() {
    let inputs = fs::read_to_string("src/input.txt").expect("Unable to read file");
    let steps = 64; // replace with your desired number of steps
    let result = work_p1(&inputs, steps)+ 1;
    println!("Number of reachable plots: {}", result);
}