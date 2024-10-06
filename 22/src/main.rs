use std::collections::{HashMap, HashSet, VecDeque};
use std::io::BufRead;
use std::io::Read;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    x1: i32, y1: i32, z1: i32,
    x2: i32, y2: i32, z2: i32,
}

impl Brick {
    fn new(coords: Vec<i32>) -> Self {
        Brick {
            x1: coords[0], y1: coords[1], z1: coords[2],
            x2: coords[3], y2: coords[4], z2: coords[5],
        }
    }

    fn overlaps(&self, other: &Brick) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 &&
        self.y1 <= other.y2 && self.y2 >= other.y1
    }

    fn lower(&mut self) {
        self.z1 -= 1;
        self.z2 -= 1;
    }
}


fn read_input(path:&str) -> Vec<Brick> {
    let file = std::io::BufReader::new(std::fs::File::open(path).unwrap());
    let mut bricks = Vec::new();
    for line in file.lines() {
        
        let coords: Vec<i32> = line.unwrap().trim().replace("~", ",")
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        bricks.push(Brick::new(coords));
    }
    bricks
}

fn settle_bricks(bricks: &mut Vec<Brick>) {
    bricks.sort_by_key(|b| b.z1);
    for i in 0..bricks.len() {
        let mut max_z = 1;
        for j in 0..i {
            if bricks[i].overlaps(&bricks[j]) {
                max_z = max_z.max(bricks[j].z2 + 1);
            }
        }
        let diff = bricks[i].z1 - max_z;
        bricks[i].z1 -= diff;
        bricks[i].z2 -= diff;
    }
}

fn find_supports(bricks: &Vec<Brick>) -> (HashMap<usize, HashSet<usize>>, HashMap<usize, HashSet<usize>>) {
    let mut supports = HashMap::new();
    let mut supported_by = HashMap::new();
    for i in 0..bricks.len() {
        for j in 0..bricks.len() {
            if i != j && bricks[i].overlaps(&bricks[j]) && bricks[i].z2 + 1 == bricks[j].z1 {
                supports.entry(i).or_insert(HashSet::new()).insert(j);
                supported_by.entry(j).or_insert(HashSet::new()).insert(i);
            }
        }
    }
    (supports, supported_by)
}

fn count_safe_to_disintegrate(bricks: &Vec<Brick>) -> usize {
    let (supports, supported_by) = find_supports(bricks);
    let mut count = 0;
    for i in 0..bricks.len() {
        if supports.get(&i).map_or(true, |upper| {
            upper.iter().all(|&j| supported_by[&j].len() > 1)
        }) {
            count += 1;
        }
    }
    count
}


fn count_chain_reaction(bricks: &Vec<Brick>) -> usize {
    let (supports, supported_by) = find_supports(bricks);
    let mut total_falling = 0;

    for i in 0..bricks.len() {
        let mut falling = HashSet::new();
        falling.insert(i);
        let mut queue = VecDeque::new();
        queue.push_back(i);

        while let Some(brick) = queue.pop_front() {
            if let Some(upper_bricks) = supports.get(&brick) {
                for &upper in upper_bricks {
                    if !falling.contains(&upper) && 
                       supported_by[&upper].iter().all(|&lower| falling.contains(&lower)) {
                        falling.insert(upper);
                        queue.push_back(upper);
                    }
                }
            }
        }

        total_falling += falling.len() - 1; // Subtract 1 to exclude the brick itself
    }

    total_falling
}


fn main() {
    let mut bricks = read_input("input.txt");
    settle_bricks(&mut bricks);

    let safe_count = count_safe_to_disintegrate(&bricks);
    println!("Part 1: {}", safe_count);

    let chain_reaction_sum = count_chain_reaction(&bricks);
    println!("Part 2: {}", chain_reaction_sum);
}
