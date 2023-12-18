use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
struct Coord {
    x: i128,
    y: i128,
}

impl Coord {
    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn mul(self, num: i128) -> Coord {
        Coord {
            x: self.x * num,
            y: self.y * num,
        }
    }
}

struct Dig {
    walls: Vec<Coord>,
}

impl Dig {
    fn new(insts: Vec<(String, i128, String)>) -> Dig {
        let mut cur = Coord { x: 0, y: 0 };
        let mut walls = vec![cur];

        let dirs = vec![
            ("R".to_string(), Coord { x: 1, y: 0 }),
            ("L".to_string(), Coord { x: -1, y: 0 }),
            ("U".to_string(), Coord { x: 0, y: -1 }),
            ("D".to_string(), Coord { x: 0, y: 1 }),
        ]
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();

        let hex_dirs = vec![
            ("0".to_string(), *dirs.get("R").unwrap()),
            ("1".to_string(), *dirs.get("D").unwrap()),
            ("2".to_string(), *dirs.get("L").unwrap()),
            ("3".to_string(), *dirs.get("U").unwrap()),
        ]
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();

        for (_, _, color) in insts {

            let n_length = i128::from_str_radix(&color[0..5], 16).unwrap();
            cur = cur.add(hex_dirs[&color[5..6]].mul(n_length));
            walls.push(cur);
        }

        Dig {walls}
    }

    fn total_dug(&self) -> i128 {
        let walls = &self.walls  ;
        let mut lace1 = 0;
        let mut lace2 = 0;
        let mut wall_count = 0;

        for i in 1..walls.len() {
            lace1 += walls[i - 1].x * walls[i].y;
            lace2 += walls[i - 1].y * walls[i].x;
            if walls[i - 1].x == walls[i].x {
                wall_count += (walls[i - 1].y - walls[i].y).abs();
            } else {
                wall_count += (walls[i - 1].x - walls[i].x).abs();
            }
        }

        lace1 += walls.last().unwrap().x * walls[0].y;
        lace2 += walls.last().unwrap().y * walls[0].x;

        (lace1 - lace2).abs() / 2 + wall_count / 2 + 1
    }
}

fn main() -> io::Result<()> {
    let mut insts = Vec::new();
    let file = File::open("src/input.txt")?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let dir = parts[0].to_string();
        let length = parts[1].parse::<i128>().unwrap();
        let color = parts[2].to_string().replace("#", "").replace("(", "").replace(")", "");
        insts.push((dir, length, color));
    }

    let digsite = Dig::new(insts);

    println!("result: {} m²", digsite.total_dug());

    Ok(())
}