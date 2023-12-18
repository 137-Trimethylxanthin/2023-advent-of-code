use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let dig_plan = read_dig_plan("src/input.txt");
    let lagoon = dig_lagoon_edge(&dig_plan);
    for row in &lagoon {
        println!("{:?}", row);
    }
    println!("-----------------");
    let lagoon = dig_out_lagoon(lagoon, &dig_plan);
    println!("-----------------");
    let cubic = calculate_cubic(lagoon);
    println!("size: {}m²", cubic);


}


fn read_dig_plan(path:&str) -> Vec<(String, usize, String)> {
    let mut dig_plan = Vec::new();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let dir = iter.next().unwrap().to_string();
        let ammo = iter.next().unwrap().parse::<usize>().unwrap();
        let color = iter.next().unwrap().to_string();
        dig_plan.push((dir, ammo, color));
    }
    dig_plan
}

fn dig_lagoon_edge(dig_plan: &Vec<(String, usize, String)>) -> Vec<Vec<char>> {
    let max_reight = get_max_reight(dig_plan);
    let max_down = get_max_down(dig_plan);
    let left_indent = get_left_indent(dig_plan);
    let up_indent = get_up_indent(dig_plan);

    let max_reight = max_reight + (left_indent -1);
    let max_down = max_down + (up_indent -1);

    let mut lagoon = vec![vec!['.'; max_reight as usize]; max_down as usize];
    let char_to_dig = '#';

    let mut x = left_indent as usize - 1;
    let mut y = up_indent as usize - 1;

    println!("x: {} y:{}", x, y);

    for (dir, ammo, _) in dig_plan {
        if dir == "R" {
            for _ in 0..*ammo {
                lagoon[y][x] = char_to_dig;
                x += 1;
            }
        } else if dir == "L" {
            for _ in 0..*ammo {
                lagoon[y][x] = char_to_dig;
                x -= 1;

            }
        } else if dir == "D" {
            for _ in 0..*ammo {
                lagoon[y][x] = char_to_dig;
                y += 1;
            }
        } else if dir == "U" {
            for _ in 0..*ammo {
                lagoon[y][x] = char_to_dig;
                y -= 1;
            }
        }
    }
    lagoon
}

fn get_max_reight(dig_plan: &Vec<(String, usize, String)>) -> i32 {
    let mut max_reight = 0;
    let mut right:i32 = 1;
    for (dir, ammo, _) in dig_plan {
        if dir == "R" {
            right += *ammo as i32;
        } else if dir == "L" {
            right -= *ammo as i32;
        }

        if right > max_reight {
            max_reight = right;
        }
    }

    max_reight
}

fn get_left_indent(dig_plan: &Vec<(String, usize, String)>) -> i32 {
    let mut left:i32 = 1;
    let mut max_left = 0;
    for (dir, ammo, _) in dig_plan {
        if dir == "L" {
            left += *ammo as i32;
        } else if dir == "R" {
            left -= *ammo as i32;
        }

        if left > max_left {
            max_left = left;
        }
    }

    if max_left < 0 {
        max_left = 0;
    }
    max_left
}

fn get_max_down (dig_plan: &Vec<(String, usize, String)>) -> i32 {
    let mut max_down = 0;
    let mut down:i32 = 1;
    for (dir, ammo, _) in dig_plan {
        if dir == "D" {
            down += *ammo as i32;
        } else if dir == "U" {
            down -= *ammo as i32;
        }

        if down > max_down {
            max_down = down;
        }
    }

    max_down
}

fn get_up_indent(dig_plan: &Vec<(String, usize, String)>) -> i32 {
    let mut up:i32 = 1;
    let mut max_up = 0;
    for (dir, ammo, _) in dig_plan {
        if dir == "U" {
            up += *ammo as i32;
        } else if dir == "D" {
            up -= *ammo as i32;
        }
        
        if up > max_up {
            max_up = up;
        }
    

    }

    if max_up < 0 {
        max_up = 0;
    }
    max_up
}

fn dig_out_lagoon(mut lagoon: Vec<Vec<char>>, dig_plan: &Vec<(String, usize, String)>) -> Vec<Vec<char>> {
    let rows = lagoon.len();
    let cols = lagoon[0].len();
    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let (mut start_x, mut start_y) = (0, 0);


    'outer: for i in 0..rows {
        for j in 0..cols {
            if lagoon[get_left_indent(dig_plan) as usize + i][get_left_indent(dig_plan) as usize + j] == '.' {
                start_x = get_left_indent(dig_plan) as usize + i;
                start_y =  get_up_indent(dig_plan) as usize + j;
                break 'outer;
            }
        }
    }

    fn dfs(x: usize, y: usize, lagoon: &mut Vec<Vec<char>>, directions: &Vec<(isize, isize)>, rows: usize, cols: usize) {
        if x >= rows || y >= cols {
            return;
        }
        if lagoon[x][y] != '.' {
            return;
        }
        lagoon[x][y] = '#';
        for &(dx, dy) in directions {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;
            dfs(new_x, new_y, lagoon, directions, rows, cols);
        }
    }
    println!("start_x: {} start_y: {}", start_x, start_y);

    dfs(start_x, start_y, &mut lagoon, &directions, rows, cols);
    lagoon
}

fn calculate_cubic(lagoon: Vec<Vec<char>>) -> usize {
    let mut cubic = 0;
    for row in lagoon {
        for col in row {
            if col == '#' {
                cubic += 1;
            }
        }
    }
    cubic
}