use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;

mod test;

fn main() {
    let games = read_file_to_bags("src/input.txt",12, 13, 14);

    let sum = get_sum_of_vec(games);

    println!("Your id sum is: {}",sum);
}

fn read_file_to_bags(path:&str,max_red:i32, max_green:i32, max_blue:i32) -> Vec<i32>{

    let mut real_ids:Vec<i32> = Vec::new();

    let file:File = File::open(path).unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("fuk");
        let parts: Vec<&str>= line.split(":").collect();
        let mut is_ok:bool = true;
        let cube_line = parts[1];
        let parts:Vec<&str> = parts[0].split(" ").collect();
        let id:i32 = parts[1].parse().expect("Failed to parse amount to i32");



        let cubesss:Vec<&str> = cube_line.split(";").collect();

        for cubess in cubesss{
            let cubes:Vec<&str> = cubess.split(",").collect();
            for cube in cubes {
                let parts: Vec<&str> = cube.trim().split(" ").collect();
                let color:&str = parts[1];
                let amount:i32 = parts[0].parse().expect("Failed to parse amount to i32");

                if color == "red" {
                    if amount > max_red{
                        is_ok = false;
                    }
                } else if color == "blue" {
                    if amount > max_blue{
                        is_ok = false;
                    }
                } else if color == "green" {
                    if amount > max_green{
                        is_ok = false;
                    }
                } else {
                    exit(1)
                }
            }

        }

        if is_ok{
            real_ids.push(id);
        }
    }

    return real_ids
}


fn get_sum_of_vec(nums:Vec<i32>) -> i32 {
    let mut out = 0;
    for v in nums {
        out += v;
    }
    return out;
}


