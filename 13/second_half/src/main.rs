use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn awnser(input: String) -> i32 {
    let mut ans_cols = 0;
    let mut ans_rows = 0;
    'outer: for s in input.split("\n\n") {
        let mut matrix: Vec<Vec<char>> = vec!();
        for row in s.split("\n") {
            if row.len() == 0 { continue }
            matrix.push(row.chars().collect());
        }

        let og_reflections = find_reflection(&matrix);
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                let og_char = matrix[i][j];
                let new_char = if og_char == '.' { '#' } else { '.' };
                matrix[i][j] = new_char;

                let new_reflections = find_reflection(&matrix).iter()
                    .filter(|nrf| !og_reflections.contains(nrf))
                    .map(|nrf| *nrf)
                    .collect::<Vec<(i32, bool)>>();

                let new_reflection = new_reflections.first();

                if let Some((num, vertical)) = new_reflection {
                    if *vertical {
                        ans_cols += num;
                    } else {
                        ans_rows += num;
                    }
                    continue 'outer;
                }

                matrix[i][j] = og_char;
            }
        }
    }
    ans_cols + 100 * ans_rows
}

fn find_reflection(matrix: &Vec<Vec<char>>) -> Vec<(i32, bool)> {
    let mut ans = vec!();

    for i in 1..(matrix[0].len() as i32) {
        let mut left: i32 = i - 1;
        let mut right: i32 = i;

        while left >= 0 && right < matrix[0].len() as i32 && (0..matrix.len()).all(|z| matrix[z][left as usize] == matrix[z][right as usize]) {
            left -= 1;
            right += 1;
        }

        if left < 0 || right == matrix[0].len() as i32 {
            ans.push((i, true));
        }
    }

    for i in 1..(matrix.len() as i32) {
        let mut up: i32 = i - 1;
        let mut down: i32 = i;

        while up >= 0 && down < matrix.len() as i32 && (0..matrix[up as usize].len()).all(|z| matrix[up as usize][z] == matrix[down as usize][z]) {
            up -= 1;
            down += 1;
        }

        if up < 0 || down == matrix.len() as i32 {
            ans.push((i, false));
        }
    }

    ans
}


fn main() -> std::io::Result<()> {
    let path = Path::new("src/input.txt");
    let input = std::fs::read_to_string(&path)?;

    let result = awnser(input);
    println!("{}", result);

    Ok(())
}