use std::collections::HashMap;
use std::fs::File;
use std::io::Read;


fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}


fn main() {
    let time = std::time::Instant::now();
    let temp = get_the_map_data("src/input.txt");
    let walking = temp.0;
    let map = temp.1;
    let steps = walk_the_map(walking, map);
    println!("steps {}", steps);
    println!("Time: {:?}", time.elapsed());
}


fn get_the_map_data(path:&str) -> (Vec<String>, HashMap<String, (String,String)>) {
    let mut map = HashMap::new();
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let parts: Vec<&str> = contents.split("\n\n").collect();

    let mut walking: Vec<String> = parts[0].split("").map(|s| s.to_string()).collect();
    walking.retain(|s| !s.is_empty());



    for direction in parts[1].split("\n") {
        let direction: Vec<&str> = direction.split(" = ").collect();
        let key = direction[0].to_string();
        let destination = direction[1].replace("(", "").replace(")", "").split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
        map.insert(key, (destination[0].clone(), destination[1].clone()));
    }

    println!("walking {:?}", walking);
    println!("map {:?}", map);


    (walking, map)
}

fn walk_the_map(walking: Vec<String>, map: HashMap<String, (String,String)>) -> u64  {
    let mut steps: Vec<u64> = Vec::new();
    let mut current_positions: Vec<String> = map.keys().filter(|k| k.ends_with("A")).cloned().collect();
    for current_position in current_positions {
        let mut step: u64 = 0;
        let mut current_position = current_position;
        while !current_position.ends_with("Z") {
            match walking[step as usize % walking.len()].as_str() {
                "R" => current_position = map.get(&current_position).unwrap().1.clone(),
                "L" => current_position = map.get(&current_position).unwrap().0.clone(),
                _ => panic!("Unexpected direction {}", walking[0].as_str()),
            }
            step += 1;
        }
        steps.push(step);
    }
    steps.iter().fold(1, |a, &b| lcm(a, b))

}

