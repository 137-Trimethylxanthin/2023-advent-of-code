use std::{f32::MIN, io::BufRead};
use z3::ast::{Ast, Int};

#[derive(Debug, Clone, Copy)]
struct Hail {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl Hail {
    fn new(x: i64, y: i64, z: i64, vx: i64, vy: i64, vz: i64) -> Hail {
        Hail { x, y, z, vx, vy, vz }
    }

    fn move_hail(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

}



fn read_input(path:&str) -> Vec<Hail> {
    let mut hails = Vec::new();
    let reader = std::io::BufReader::new(std::fs::File::open(path).unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        //split , and @
        let parts: Vec<&str> = line.split(|c| c == ',' || c == '@').collect();
        let x = parts[0].trim().parse().unwrap();
        let y = parts[1].trim().parse().unwrap();
        let z = parts[2].trim().parse().unwrap();
        let vx = parts[3].trim().parse().unwrap();
        let vy = parts[4].trim().parse().unwrap();
        let vz = parts[5].trim().parse().unwrap();
        hails.push(Hail::new(x, y, z, vx, vy, vz));
    }

    hails

}


fn find_intersection(hailA: Hail, hailB: Hail) -> Option<(f64, f64)> {
    let det: f64 = (hailA.vx as f64 * hailB.vy as f64 - hailA.vy as f64 * hailB.vx as f64);

    // Wenn det = 0, dann sind die Strahlen parallel
    if det.abs() < 1e-10 {
        return None;
    }

    let dx = hailB.x as f64 - hailA.x as f64;
    let dy = hailB.y as f64 - hailA.y as f64;

    let t1 = (dx * hailB.vy as f64 - dy * hailB.vx as f64) / det;
    let t2 = (dx * hailA.vy as f64 - dy * hailA.vx as f64) / det;

    // Prüfe, ob t1 und t2 beide >= 0 sind (nur dann schneiden sich die Strahlen)
    if t1 >= 0.0 && t2 >= 0.0 {
        let intersection_x = hailA.x as f64 + t1 * hailA.vx as f64;
        let intersection_y = hailA.y as f64 + t1 * hailA.vy as f64;
        Some((intersection_x, intersection_y))
    } else {
        None
    }
}


fn find_collissions(hail: &Vec<Hail>, min: i64, max: i64) -> Vec<(f64, f64)> {
    let mut collisions = Vec::new();
    for i in 0..hail.len() {
        for j in i+1..hail.len() {
            //check if the two hails are within the max collision area
            if let Some((x, y)) = find_intersection(hail[i], hail[j]) {
                if x >= min as f64 && x <= max as f64 && y >= min as f64 && y <= max as f64 {
                    collisions.push((x, y));
                }
            }
        }
    }
    collisions
}



fn solve_part_two(hails: &Vec<Hail>) -> i64 {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for (i, hail) in hails.iter().enumerate().take(3) {
        let t = Int::new_const(&ctx, format!("t{}", i));
        solver.assert(&t.ge(&Int::from_i64(&ctx, 0)));

        let hx = Int::from_i64(&ctx, hail.x);
        let hy = Int::from_i64(&ctx, hail.y);
        let hz = Int::from_i64(&ctx, hail.z);
        let hvx = Int::from_i64(&ctx, hail.vx);
        let hvy = Int::from_i64(&ctx, hail.vy);
        let hvz = Int::from_i64(&ctx, hail.vz);

        solver.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(hx + hvx * t.clone())));
        solver.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(hy + hvy * t.clone())));
        solver.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(hz + hvz * t.clone())));
    }

    if solver.check() == z3::SatResult::Sat {
        let model = solver.get_model().unwrap();
        let x_val = model.eval(&x, true).unwrap().as_i64().unwrap();
        let y_val = model.eval(&y, true).unwrap().as_i64().unwrap();
        let z_val = model.eval(&z, true).unwrap().as_i64().unwrap();
        println!("x: {}, y: {}, z: {}", x_val, y_val, z_val);
        x_val + y_val + z_val
    } else {
        panic!("No solution found");
    }
}


fn main() {
    let input = read_input("input.txt");
    let min: i64 = 200000000000000; // 7
    let max: i64 = 400000000000000; // 27


    let collisions = find_collissions(&input, min, max);

    let collision_count = collisions.len();
    for (i, collision) in collisions.iter().enumerate() {
        println!("Collision {}: ({}, {})", i, collision.0, collision.1);
    }
    println!("Number of collisions (part 1): {}", collision_count);

    let part_two_result = solve_part_two(&input);
    println!("Sum of initial position coordinates (part 2): {}", part_two_result);


}
