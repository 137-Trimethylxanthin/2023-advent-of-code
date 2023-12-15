use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    let vec_steps = read_file_to_vec("src/input.txt");
    let boxs = generate_box(vec_steps);
    let sum = total_focusing_power(&boxs);


    println!("sum: {}", sum);
}

fn read_file_to_vec(filename: &str) -> Vec<String> {

    let mut file = File::open(filename).expect("File not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not read file");


    content.split(",").map(|s| s.to_string()).collect()
}


fn parse_step(step: &str) -> (String, char, Option<u32>) {
    let parts: Vec<&str> = step.splitn(3, |c| c == '=' || c == '-').collect();
    let label = parts[0].to_string();
    let operation = step.find('=').map(|_| '=').unwrap_or('-');
    let focal_length = if operation == '=' {
        Some(parts[1].parse().unwrap())
    } else {
        None
    };
    (label, operation, focal_length)
}

fn generate_box(vec_steps: Vec<String>) -> Vec<Vec<(String, u32)>> {
    let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![]; 256];
    for step in vec_steps {
        let (label, operation, focal_length) = parse_step(&step);
        let box_number = Holiday_ASCII_String_Helper_algorithm(&label) as usize;
        match operation {
            '-' => {
                boxes[box_number].retain(|lens| lens.0 != label);
            }
            '=' => {
                if let Some(lens) = boxes[box_number].iter_mut().find(|lens| lens.0 == label) {
                    lens.1 = focal_length.unwrap();
                } else {
                    boxes[box_number].push((label, focal_length.unwrap()));
                }
            }
            _ => {}
        }
    }
    boxes
}

fn total_focusing_power(boxes: &Vec<Vec<(String, u32)>>) -> u32 {
    boxes.iter().enumerate().map(|(box_number, b)| {
        b.iter().enumerate().map(|(slot_number, lens)| {
            (box_number as u32 + 1) * (slot_number as u32 + 1) * lens.1
        }).sum::<u32>()
    }).sum()
}



fn Holiday_ASCII_String_Helper_algorithm(string: &str) -> u32 {
    let mut result = 0;
    for c in string.chars() {
        result += c as u32;
        result *= 17;
        result %= 256;
    }

    result

}