use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;

mod test;
fn main() {
    let games = read_file_to_bags("src/input.txt");

    let sum = get_sum_of_vec(games);

    println!("Your id sum is: {}",sum);
}

fn read_file_to_bags(path:&str) -> Vec<i32>{

    let mut powers:Vec<i32> = Vec::new();

    let file:File = File::open(path).unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("fuk");
        let parts: Vec<&str>= line.split(":").collect();
        let mut is_ok:bool = true;
        let cube_line = parts[1];
        let parts:Vec<&str> = parts[0].split(" ").collect();

        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green= 0;


        let cubesss:Vec<&str> = cube_line.split(";").collect();

        for cubess in cubesss{
            let cubes:Vec<&str> = cubess.split(",").collect();
            for cube in cubes {
                let parts: Vec<&str> = cube.trim().split(" ").collect();
                let color:&str = parts[1];
                let amount:i32 = parts[0].parse().expect("Failed to parse amount to i32");

                if color == "red" {
                    if amount > max_red{
                        max_red = amount
                    }
                } else if color == "blue" {
                    if amount > max_blue{
                        max_blue = amount
                    }
                } else if color == "green" {
                    if amount > max_green{
                        max_green = amount
                    }
                } else {
                    exit(1)
                }
            }

        }



        powers.push((max_red*max_green*max_blue));

    }

    return powers
}


fn get_sum_of_vec(nums:Vec<i32>) -> i32 {
    let mut out = 0;
    for v in nums {
        out += v;
    }
    return out;
}


