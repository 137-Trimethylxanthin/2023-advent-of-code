use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

type Point = (usize, usize, char);

fn read_file(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn process_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn traverse_map(
    map: &Vec<Vec<char>>,
    movement: Point,
    visited: &mut Vec<Vec<i32>>,
    cache: &mut HashSet<Point>,
) {
    let (x, y, direction) = movement;

    if y >= map.len() || x >= map[0].len() {
        return;
    }

    if cache.contains(&movement) {
        return;
    }

    cache.insert(movement);
    visited[y][x] = 1;
    let tile = map[y][x];

    match tile {
        '|' => {
            traverse_map(map, (x, y + 1, 'U'), visited, cache);
            traverse_map(map, (x, y - 1, 'D'), visited, cache);
        }
        '-' => {
            traverse_map(map, (x + 1, y, 'R'), visited, cache);
            traverse_map(map, (x - 1, y, 'L'), visited, cache);
        }
        '\\' => match direction {
            'R' => traverse_map(map, (x, y + 1, 'U'), visited, cache),
            'L' => traverse_map(map, (x, y - 1, 'D'), visited, cache),
            'U' => traverse_map(map, (x + 1, y, 'R'), visited, cache),
            'D' => traverse_map(map, (x - 1, y, 'L'), visited, cache),
            _ => (),
        },
        '/' => match direction {
            'R' => traverse_map(map, (x, y - 1, 'D'), visited, cache),
            'L' => traverse_map(map, (x, y + 1, 'U'), visited, cache),
            'U' => traverse_map(map, (x - 1, y, 'L'), visited, cache),
            'D' => traverse_map(map, (x + 1, y, 'R'), visited, cache),
            _ => (),
        },
        '.' => match direction {
            'R' => traverse_map(map, (x + 1, y, 'R'), visited, cache),
            'L' => traverse_map(map, (x - 1, y, 'L'), visited, cache),
            'U' => traverse_map(map, (x, y + 1, 'U'), visited, cache),
            'D' => traverse_map(map, (x, y - 1, 'D'), visited, cache),
            _ => (),
        },
        _ => (),
    }
}

fn calc_visited_nodes(map: &Vec<Vec<char>>, starting: Point) -> i32 {
    let mut cache = HashSet::new();
    let mut visited = vec![vec![0; map[0].len()]; map.len()];
    traverse_map(map, starting, &mut visited, &mut cache);
    visited.iter().flatten().sum()
}

fn solution(map: &Vec<Vec<char>>) -> i32 {
    let mut max_val = 0;
    for x in 0..map[0].len() {
        max_val = max_val.max(calc_visited_nodes(map, (x, 0, 'D')));
        max_val = max_val.max(calc_visited_nodes(map, (x, map.len() - 1, 'U')));
    }
    for y in 0..map.len() {
        max_val = max_val.max(calc_visited_nodes(map, (0, y, 'R')));
        max_val = max_val.max(calc_visited_nodes(map, (map[0].len() - 1, y, 'L')));
    }
    max_val
}

fn main() -> std::io::Result<()> {
    let input = read_file("src/input.txt")?;
    let data = process_input(&input);

    let solution = solution(&data);

    println!("Solution: {}", solution);

    Ok(())
}
