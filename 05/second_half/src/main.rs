use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::process::exit;
use std::str::FromStr;
use regex::Regex;

mod test;

fn extract_numbers(s: &str) -> Vec<i64> {
    let re = Regex::new(r"\d+").unwrap();
    re.captures_iter(s)
        .map(|cap| i64::from_str(&cap[0]).unwrap())
        .collect()
}

fn main() {
    println!("Hello, world!");
    let v = read_file_to_vec("src/input.txt");
    println!("{:?}", v);
}

fn read_file_to_vec(path:&str) -> i64 {
    let file_content = fs::read_to_string(path).unwrap();

    let part:Vec<&str> = file_content.split(":").collect();
    let seed = part[1];
    let seed_ranges = seed.split_whitespace().collect::<Vec<&str>>();

    let seed_to_soil: &str = part[2].trim();
    let seed_to_soil_vec = map_extraction(seed_to_soil);

    let soil_to_fertilizer: &str = part[3].trim();
    let soil_to_fertilizer_vec = map_extraction(soil_to_fertilizer);

    let fertilizer_to_water: &str = part[4].trim();
    let fertilizer_to_water_vec = map_extraction(fertilizer_to_water);

    let water_to_light: &str = part[5].trim();
    let water_to_light_vec = map_extraction(water_to_light);

    let light_to_temperature: &str = part[6].trim();
    let light_to_temperature_vec = map_extraction(light_to_temperature);

    let temperature_to_humidity: &str = part[7].trim();
    let temperature_to_humidity_vec = map_extraction(temperature_to_humidity);

    let humidity_to_location: &str = part[8].trim();
    let humidity_to_location_vec = map_extraction(humidity_to_location);





    //println!("{:?}", seed_vec);
    println!("{:?}", seed_to_soil_vec);
    println!("{:?}", soil_to_fertilizer_vec);
    println!("{:?}", fertilizer_to_water_vec);
    println!("{:?}", water_to_light_vec);
    println!("{:?}", light_to_temperature_vec);
    println!("{:?}", temperature_to_humidity_vec);
    println!("{:?}", humidity_to_location_vec);

    let mut locations: Vec<i64> = Vec::new();

    for i in (0..seed_ranges.len()).step_by(2) {
        let start = seed_ranges[i].parse::<i64>().unwrap_or(0);
        let len = seed_ranges[i+1].parse::<i64>().unwrap_or(0);
        for num in start..(start+len) {
            let soil = convert_number(num, &seed_to_soil_vec);
            let fertilizer = convert_number(soil, &soil_to_fertilizer_vec);
            let water = convert_number(fertilizer, &fertilizer_to_water_vec);
            let light = convert_number(water, &water_to_light_vec);
            let temperature = convert_number(light, &light_to_temperature_vec);
            let humidity = convert_number(temperature, &temperature_to_humidity_vec);
            let location = convert_number(humidity, &humidity_to_location_vec);
            locations.push(location);
        }
    }

    let min_location = locations.iter().min().unwrap();
    println!("The lowest location number is {}", min_location);

    0
}
fn convert_number(num: i64, map: &Vec<(i64, i64, i64)>) -> i64 {
    for &(dest_start, src_start, len) in map {
        if num >= src_start && num < src_start + len {
            return dest_start + (num - src_start);
        }
    }
    num
}

fn map_extraction(map: &str) -> Vec<(i64, i64, i64)> {
    let part = map.split("\n").collect::<Vec<&str>>();
    let mut vec: Vec<(i64, i64, i64)> = Vec::new();
    for i in 0..part.len() {
        let soil = extract_numbers(part[i]);
        if soil.len() == 3 {
            let tuple = (soil[0], soil[1], soil[2]);
            vec.push(tuple);
        }
    }
    vec
}
