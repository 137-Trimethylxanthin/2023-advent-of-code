use std::{fs::File, io::{BufRead, BufReader}};

struct Map {
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
}

impl Map {
    fn new(width: u32, height: u32, map_data: Vec<char>) -> Map {
        let mut tiles = Vec::new();
        for (i, c) in map_data.iter().enumerate() {
            let x = i as u32 % width;
            let y = i as u32 / width;
            let tile_type = match c {
                '.' => TileType::Path,
                '#' => TileType::Forest,
                'v' => TileType::DSlope,
                '^' => TileType::USlope,
                '<' => TileType::LSlope,
                '>' => TileType::RSlope,
                'S' => TileType::Start,
                'E' => TileType::End,
                _ => panic!("Unknown tile type: {}", c),
            };
            tiles.push(Tile { x, y, tile_type });
        }
        Map {
            width,
            height,
            tiles,
        }
    }

    fn not_slippery(&mut self) {
        // Replace all slopes with path
        for tile in &mut self.tiles {
            match tile.tile_type {
                TileType::DSlope | TileType::USlope | TileType::LSlope | TileType::RSlope => {
                    tile.modify_tile_type(TileType::Path);
                }
                _ => {}
            }
        }
    }

    
}

struct Tile {
    x: u32,
    y: u32,
    tile_type: TileType,
}

impl Tile {
    fn new(x: u32, y: u32, tile_type: TileType) -> Tile {
        Tile {
            x,
            y,
            tile_type,
        }
    }

    fn modify_tile_type(&mut self, new_type: TileType) {
        self.tile_type = new_type;
    }
    
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TileType {
    Path,
    Forest,

    DSlope,
    USlope,
    LSlope,
    RSlope,
    
    Start,
    End,
}



struct Player {
    x: u32,
    y: u32,
    steps: u32,
    already_visited: Vec<Vec<bool>>,
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            x: self.x,
            y: self.y,
            steps: self.steps,
            already_visited: self.already_visited.clone(),
        }
    }
}

impl Player {
    fn new(x: u32, y: u32, width: u32, height: u32) -> Player {
        let mut already_visited = vec![vec![false; width as usize]; height as usize];

        already_visited[y as usize][x as usize] = true;
        
        Player {
            x,
            y,
            steps: 0,
            already_visited: already_visited,
        }
    }

    fn movePlayer(&mut self, map: &Map, directrion: (i32, i32)) -> TileType {
        let new_x = self.x as i32 + directrion.0;
        let new_y = self.y as i32 + directrion.1;
        if new_x < 0 || new_x >= map.width as i32 || new_y < 0 || new_y >= map.height as i32 {
            return TileType::Forest;
        }
        let new_x = new_x as u32;
        let new_y = new_y as u32;
        if self.already_visited[new_y as usize][new_x as usize] {
            return TileType::Forest;
        }
        let tile = map.tiles.iter().find(|t| t.x == new_x && t.y == new_y).unwrap();

        match tile.tile_type {
            TileType::Path => {
                self.x = new_x;
                self.y = new_y;
                self.steps += 1;
                self.already_visited[new_y as usize][new_x as usize] = true;
            }
            TileType::Forest => {
                return TileType::Forest;
            }
            TileType::DSlope => {
                if directrion == (0, 1) {
                    self.already_visited[new_y as usize][new_x as usize] = true;
                    self.x = new_x;
                    self.y = new_y;
                    self.steps += 1;
                    self.movePlayer(map, (0, 1));
                } else {
                    return TileType::Forest;
                }
            }
            TileType::USlope => {
                if directrion == (0, -1) {
                    self.x = new_x;
                    self.y = new_y;
                    self.steps += 1;
                    self.already_visited[new_y as usize][new_x as usize] = true;
                    self.movePlayer(map, (0, -1));
                } else {
                    return TileType::Forest;
                }
            }
            TileType::LSlope => {
                if directrion == (-1, 0) {
                    self.x = new_x;
                    self.y = new_y;
                    self.steps += 1;
                    self.already_visited[new_y as usize][new_x as usize] = true;
                    self.movePlayer(map, (-1, 0));
                } else {
                    return TileType::Forest;
                }
            }
            TileType::RSlope => {
                if directrion == (1, 0) {
                    self.x = new_x;
                    self.y = new_y;
                    self.steps += 1;
                    self.already_visited[new_y as usize][new_x as usize] = true;
                    self.movePlayer(map, (1, 0));
                } else {
                    return TileType::Forest;
                }
            }
            TileType::Start => {
                // do nothing
                return TileType::Start;
            }
            TileType::End => {
                self.steps += 1;
                return TileType::End;
            }
        }
        return TileType::Path;
    }

    

    fn count_possible_moves(&self, map : &Map, x: u32, y: u32) -> u8 {
        let mut count = 0;
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)];
        for direction in directions {
            let new_x = x as i32 + direction.0;
            let new_y = y as i32 + direction.1;
            if new_x < 0 || new_x >= map.width as i32 || new_y < 0 || new_y >= map.height as i32 {
                continue;
            }
            let new_x = new_x as u32;
            let new_y = new_y as u32;
            if self.already_visited[new_y as usize][new_x as usize] {
                continue;
            }
            let tile = map.tiles.iter().find(|t| t.x == new_x && t.y == new_y).unwrap();
            println!("tile: {:?}", tile.tile_type);
            match tile.tile_type {
                TileType::Path => {
                    count += 1;
                }
                TileType::Forest => {
                    // do nothing
                }
                TileType::DSlope => {
                    if direction == (0, -1) {
                        count += 1;
                    }
                }
                TileType::USlope => {
                    if direction == (0, 1) {
                        count += 1;
                    }
                }
                TileType::LSlope => {
                    if direction == (-1, 0) {
                        count += 1;
                    }
                }
                TileType::RSlope => {
                    if direction == (1, 0) {
                        count += 1;
                    }
                }
                TileType::Start => {
                    // do nothing
                }
                TileType::End => {
                    // do nothing
                }
            }
        }
        count
    }




    
}


fn load_map(path: &str) -> Map {
    let bufreader = BufReader::new(File::open(path).unwrap());
    let lines = bufreader.lines();
    let mut map_data = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for line in lines {
        let line = line.unwrap();
        if width == 0 {
            width = line.len() as u32;
        } else {
            assert_eq!(width, line.len() as u32);
        }
        height += 1;
        map_data.extend(line.chars());
    }

    //replace first . with S and last . with E
    let start = map_data.iter().position(|&c| c == '.').unwrap();
    map_data[start] = 'S';
    let end = map_data.iter().rposition(|&c| c == '.').unwrap();
    map_data[end] = 'E';


    Map::new(width, height, map_data)
}


// if you get on a Slope, you can only move in the direction of the slope
//you cant pass through a forest
//no diagonal movement
//no dubble visit
//you know the start and end point
fn paths(map : &Map ) -> Vec<u32> {
    let start = map.tiles.iter().find(|t| t.tile_type == TileType::Start).unwrap();
    let mut players = Vec::new();
    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut all_paths = Vec::new();
    players.push(Player::new(start.x, start.y, map.width, map.height));
    while !players.is_empty() {
        //println!("players: {}", players.len());
        let mut new_players = Vec::new();
        for player in players {
            //println!("start position: {}, {}", player.x, player.y);
            for direction in &directions {
                let mut new_player = player.clone();
                let tile = new_player.movePlayer(map, *direction);
                if tile == TileType::End {
                    //println!("found end");
                    all_paths.push(new_player.steps);
                } else if tile == TileType::Forest {
                    // do nothing
                } else {
                    //println!("position: {}, {} | {}", new_player.x, new_player.y, new_player.count_possible_moves(map, new_player.x, new_player.y));
                    new_players.push(new_player);
                    
                }
            }
        }
        players = new_players;
        //println!("players: {:?}", players.len());
    }
    
    all_paths
}

fn main() {
    let mut map = load_map("input.txt");
    println!("Map: {}x{}", map.width, map.height);
    let all_paths = paths(&map);
    println!("paths: {:?}", all_paths);
    let max_steps = all_paths.iter().max().unwrap();
    println!("max steps (part 1): {}", max_steps);


    //very slow ... VERY SLOW
    map.not_slippery();
    println!("Map: {}x{}", map.width, map.height);
    let all_paths_p2:Vec<u32> = paths(&map);
    let max_steps_p2 = all_paths_p2.iter().max().unwrap();
    println!("max steps (part 2): {}", max_steps_p2);

}


