// https://adventofcode.com/2021/day/23
use crate::common::Solution;
use std::str::FromStr;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
enum Tile {
    Empty,
    Occupied(Amphipod),
}

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn energy_per_step(&self) -> usize {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }
}

const OUTER_LEFT_CORNER: usize = 0;
const INNER_LEFT_CORNER: usize = 1;
const AMBER_ROOM_1: usize = 2;
const AMBER_ROOM_4: usize = 5;
const LEFT_HALLWAY: usize = 6;
const BRONZE_ROOM_1: usize = 7;
const BRONZE_ROOM_4: usize = 10;
const MIDDLE_HALLWAY: usize = 11;
const COPPER_ROOM_1: usize = 12;
const COPPER_ROOM_4: usize = 15;
const RIGHT_HALLWAY: usize = 16;
const DESERT_ROOM_1: usize = 17;
const DESERT_ROOM_4: usize = 20;
const INNER_RIGHT_CORNER: usize = 21;
const OUTER_RIGHT_CORNER: usize = 22;

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
struct GameState {
    // 01 6 b 0 5 6
    //   2 7 c 1
    //   3 8 d 2
    //   4 9 e 3
    //   5 a f 4
    // 16+7 = 23 valid positions in the grid
    grid: [Tile;23],

    // 23 locations, with 5 possible values for each location.
    // 3 bits per location => 69 bits. 
    // 5*5
    // 5 bits per two locations => 55 + 5 = 60 bits. 
}

impl GameState {

    fn destination() -> GameState {
        GameState { grid: 
        [Tile::Empty, Tile::Empty, 
        Tile::Occupied(Amphipod::Amber), Tile::Occupied(Amphipod::Amber), Tile::Occupied(Amphipod::Amber), Tile::Occupied(Amphipod::Amber), 
        Tile::Empty,
        Tile::Occupied(Amphipod::Bronze), Tile::Occupied(Amphipod::Bronze), Tile::Occupied(Amphipod::Bronze), Tile::Occupied(Amphipod::Bronze), 
        Tile::Empty,
        Tile::Occupied(Amphipod::Copper), Tile::Occupied(Amphipod::Copper), Tile::Occupied(Amphipod::Copper), Tile::Occupied(Amphipod::Copper), 
        Tile::Empty,
        Tile::Occupied(Amphipod::Desert), Tile::Occupied(Amphipod::Desert), Tile::Occupied(Amphipod::Desert), Tile::Occupied(Amphipod::Desert), 
        Tile::Empty, Tile::Empty
        ] }
    }

    fn tile_is_in_a_room(tile: usize) -> bool {
        match tile {
            AMBER_ROOM_1..=AMBER_ROOM_4 |
            BRONZE_ROOM_1..=BRONZE_ROOM_4 |
            COPPER_ROOM_1..=COPPER_ROOM_4 |
            DESERT_ROOM_1..=DESERT_ROOM_4 => true,
            _ => false,
        }
    }

    fn is_happy(&self, index: usize) -> bool {
        if GameState::tile_is_in_a_room(index) {
            match self.grid[index] {
                Tile::Empty => true,
                Tile::Occupied(Amphipod::Amber) if index >= AMBER_ROOM_1 && index <= AMBER_ROOM_4 && self.grid[index..=AMBER_ROOM_4].iter().all(|&s| s == Tile::Occupied(Amphipod::Amber)) => true,
                Tile::Occupied(Amphipod::Bronze) if index >= BRONZE_ROOM_1 && index <= BRONZE_ROOM_4 && self.grid[index..=BRONZE_ROOM_4].iter().all(|&s| s == Tile::Occupied(Amphipod::Bronze)) => true,
                Tile::Occupied(Amphipod::Copper) if index >= COPPER_ROOM_1 && index <= COPPER_ROOM_4 && self.grid[index..=COPPER_ROOM_4].iter().all(|&s| s == Tile::Occupied(Amphipod::Copper)) => true,
                Tile::Occupied(Amphipod::Desert) if index >= DESERT_ROOM_1 && index <= DESERT_ROOM_4 && self.grid[index..=DESERT_ROOM_4].iter().all(|&s| s == Tile::Occupied(Amphipod::Desert)) => true,
                _ => false,
            }
        } else {
            false
        }
    }
    fn is_empty(&self, i: usize) -> bool {
        self.grid[i] == Tile::Empty
    }

    fn free_path(&self, from: usize, to: usize) -> Option<usize> {
        match from {
            OUTER_LEFT_CORNER => {
                if self.is_empty(INNER_LEFT_CORNER) {
                    match to {
                        AMBER_ROOM_1..=AMBER_ROOM_4 => Some(3+to-AMBER_ROOM_1),
                        BRONZE_ROOM_1..=BRONZE_ROOM_4 if self.is_empty(LEFT_HALLWAY) => Some(5+to-BRONZE_ROOM_1),
                        COPPER_ROOM_1..=COPPER_ROOM_4 if self.is_empty(LEFT_HALLWAY) && 
                                                         self.is_empty(MIDDLE_HALLWAY) => Some(7+to-COPPER_ROOM_1),
                        DESERT_ROOM_1..=DESERT_ROOM_4 if self.is_empty(LEFT_HALLWAY) && 
                                                         self.is_empty(MIDDLE_HALLWAY) && 
                                                         self.is_empty(RIGHT_HALLWAY) => Some(9+to-DESERT_ROOM_1),
                        _ => None,                
                    }
                } else {
                    None
                }
            },
            INNER_LEFT_CORNER => {
                match to {
                    AMBER_ROOM_1..=AMBER_ROOM_4 => Some(2+to-AMBER_ROOM_1),
                    BRONZE_ROOM_1..=BRONZE_ROOM_4 if self.is_empty(LEFT_HALLWAY) => Some(4+to-BRONZE_ROOM_1),
                    COPPER_ROOM_1..=COPPER_ROOM_4 if self.is_empty(LEFT_HALLWAY) && 
                                                     self.is_empty(MIDDLE_HALLWAY) => Some(6+to-COPPER_ROOM_1),
                    DESERT_ROOM_1..=DESERT_ROOM_4 if self.is_empty(LEFT_HALLWAY) && 
                                                     self.is_empty(MIDDLE_HALLWAY) && 
                                                     self.is_empty(RIGHT_HALLWAY) => Some(8+to-DESERT_ROOM_1),
                    _ => None,                
                }
            },
            LEFT_HALLWAY => {
                match to {
                    AMBER_ROOM_1..=AMBER_ROOM_4 => Some(2+to-AMBER_ROOM_1),
                    BRONZE_ROOM_1..=BRONZE_ROOM_4 => Some(2+to-BRONZE_ROOM_1),
                    COPPER_ROOM_1..=COPPER_ROOM_4 if self.is_empty(MIDDLE_HALLWAY) => Some(4+to-COPPER_ROOM_1),
                    DESERT_ROOM_1..=DESERT_ROOM_4 if self.is_empty(MIDDLE_HALLWAY) && 
                                                     self.is_empty(RIGHT_HALLWAY) => Some(6+to-DESERT_ROOM_1), 
                    _ => None,
                }
            },
            MIDDLE_HALLWAY => {
                match to {
                    AMBER_ROOM_1..=AMBER_ROOM_4 if self.is_empty(MIDDLE_HALLWAY) => Some(4+to-AMBER_ROOM_1),
                    BRONZE_ROOM_1..=BRONZE_ROOM_4 => Some(2+to-BRONZE_ROOM_1),
                    COPPER_ROOM_1..=COPPER_ROOM_4 => Some(2+to-COPPER_ROOM_1),
                    DESERT_ROOM_1..=DESERT_ROOM_4 if self.is_empty(RIGHT_HALLWAY) => Some(4+to-DESERT_ROOM_1), 
                    _ => None,
                }
            },
            RIGHT_HALLWAY => {
                match to {
                    AMBER_ROOM_1..=AMBER_ROOM_4 if self.is_empty(MIDDLE_HALLWAY) && 
                                                   self.is_empty(LEFT_HALLWAY) => Some(6+to-AMBER_ROOM_1),
                    BRONZE_ROOM_1..=BRONZE_ROOM_4 if self.is_empty(MIDDLE_HALLWAY) => Some(4+to-BRONZE_ROOM_1),
                    COPPER_ROOM_1..=COPPER_ROOM_4 => Some(2+to-COPPER_ROOM_1),
                    DESERT_ROOM_1..=DESERT_ROOM_4 => Some(2+to-DESERT_ROOM_1), 
                    _ => None,
                }
            },
            INNER_RIGHT_CORNER => {
                match to {
                    AMBER_ROOM_1..=AMBER_ROOM_4 if self.is_empty(LEFT_HALLWAY) && 
                                                   self.is_empty(MIDDLE_HALLWAY) && 
                                                   self.is_empty(RIGHT_HALLWAY) => Some(8+to-AMBER_ROOM_1),
                    BRONZE_ROOM_1..=BRONZE_ROOM_4 if self.is_empty(MIDDLE_HALLWAY) && 
                                                   self.is_empty(RIGHT_HALLWAY) => Some(6+to-BRONZE_ROOM_1),
                    COPPER_ROOM_1..=COPPER_ROOM_4 if self.is_empty(RIGHT_HALLWAY) => Some(4+to-COPPER_ROOM_1),
                    DESERT_ROOM_1..=DESERT_ROOM_4 => Some(2+to-DESERT_ROOM_1) ,
                    _ => None,                
                }
            },
            OUTER_RIGHT_CORNER => {
                if self.is_empty(INNER_RIGHT_CORNER) {
                    match to {
                        AMBER_ROOM_1..=AMBER_ROOM_4 if self.is_empty(INNER_LEFT_CORNER) => Some(9+to-AMBER_ROOM_1),
                        BRONZE_ROOM_1..=BRONZE_ROOM_4 if self.is_empty(INNER_LEFT_CORNER) && 
                                                        self.is_empty(LEFT_HALLWAY) => Some(7+to-BRONZE_ROOM_1),
                        COPPER_ROOM_1..=COPPER_ROOM_4 if self.is_empty(INNER_LEFT_CORNER) && 
                                                        self.is_empty(LEFT_HALLWAY) && 
                                                        self.is_empty(MIDDLE_HALLWAY) => Some(5+to-COPPER_ROOM_1),
                        DESERT_ROOM_1..=DESERT_ROOM_4 if self.is_empty(INNER_LEFT_CORNER) && 
                                                        self.is_empty(LEFT_HALLWAY) && 
                                                        self.is_empty(MIDDLE_HALLWAY) && 
                                                        self.is_empty(RIGHT_HALLWAY) => Some(3+to-DESERT_ROOM_1),
                        _ => None,                
                    }
                } else {
                    None
                }
            },
            // Caller is responsible for checking that the way out of the room is clear
            AMBER_ROOM_1..=AMBER_ROOM_4 => {
                match to {
                    OUTER_LEFT_CORNER if self.is_empty(INNER_LEFT_CORNER) => Some(3 + from - AMBER_ROOM_1),
                    INNER_LEFT_CORNER | LEFT_HALLWAY => Some(2 + from - AMBER_ROOM_1),
                    MIDDLE_HALLWAY if self.is_empty(LEFT_HALLWAY) => Some(4 + from - AMBER_ROOM_1),
                    RIGHT_HALLWAY if self.is_empty(LEFT_HALLWAY) && self.is_empty(MIDDLE_HALLWAY) => Some(6 + from - AMBER_ROOM_1),
                    INNER_RIGHT_CORNER if self.is_empty(LEFT_HALLWAY) && self.is_empty(MIDDLE_HALLWAY) && self.is_empty(RIGHT_HALLWAY) => Some(8 + from - AMBER_ROOM_1),
                    OUTER_RIGHT_CORNER if self.is_empty(LEFT_HALLWAY) && self.is_empty(MIDDLE_HALLWAY) && 
                                          self.is_empty(RIGHT_HALLWAY) && self.is_empty(INNER_RIGHT_CORNER) => Some(9 + from - AMBER_ROOM_1),
                    _ => None,
                }
            },
            BRONZE_ROOM_1..=BRONZE_ROOM_4 => {
                match to {
                    OUTER_LEFT_CORNER if self.is_empty(INNER_LEFT_CORNER) && self.is_empty(LEFT_HALLWAY) => Some(5 + from - BRONZE_ROOM_1),
                    INNER_LEFT_CORNER if self.is_empty(LEFT_HALLWAY) => Some(4 + from - BRONZE_ROOM_1),
                    LEFT_HALLWAY | MIDDLE_HALLWAY => Some(2+from-BRONZE_ROOM_1),
                    RIGHT_HALLWAY if self.is_empty(MIDDLE_HALLWAY) => Some(4+from-BRONZE_ROOM_1),
                    INNER_RIGHT_CORNER if self.is_empty(MIDDLE_HALLWAY) && self.is_empty(RIGHT_HALLWAY) => Some(6 + from - BRONZE_ROOM_1),
                    OUTER_RIGHT_CORNER if self.is_empty(MIDDLE_HALLWAY) && 
                                          self.is_empty(RIGHT_HALLWAY) && self.is_empty(INNER_RIGHT_CORNER) => Some(7 + from - BRONZE_ROOM_1),
                    _ => None,
                }
            },
            COPPER_ROOM_1..=COPPER_ROOM_4 => {
                match to {
                    OUTER_LEFT_CORNER if self.is_empty(INNER_LEFT_CORNER) && self.is_empty(LEFT_HALLWAY) && self.is_empty(MIDDLE_HALLWAY) => Some(7 + from - COPPER_ROOM_1),
                    INNER_LEFT_CORNER if self.is_empty(LEFT_HALLWAY) && self.is_empty(MIDDLE_HALLWAY) => Some(6 + from - COPPER_ROOM_1),
                    LEFT_HALLWAY if self.is_empty(MIDDLE_HALLWAY) => Some(4+from-COPPER_ROOM_1),
                    MIDDLE_HALLWAY | RIGHT_HALLWAY => Some(2+from-COPPER_ROOM_1),                    
                    INNER_RIGHT_CORNER if self.is_empty(RIGHT_HALLWAY) => Some(4 + from - COPPER_ROOM_1),
                    OUTER_RIGHT_CORNER if self.is_empty(RIGHT_HALLWAY) && self.is_empty(INNER_RIGHT_CORNER) => Some(5 + from - COPPER_ROOM_1),
                    _ => None,
                }
            },
            DESERT_ROOM_1..=DESERT_ROOM_4 => {
                match to {
                    OUTER_LEFT_CORNER if self.is_empty(INNER_LEFT_CORNER) && self.is_empty(LEFT_HALLWAY) && self.is_empty(MIDDLE_HALLWAY) && 
                                         self.is_empty(RIGHT_HALLWAY) && self.is_empty(INNER_LEFT_CORNER) => Some(9 + from - DESERT_ROOM_1),
                    INNER_LEFT_CORNER if self.is_empty(LEFT_HALLWAY) && self.is_empty(MIDDLE_HALLWAY) && 
                                         self.is_empty(RIGHT_HALLWAY) => Some(8 + from - DESERT_ROOM_1),
                    LEFT_HALLWAY if self.is_empty(MIDDLE_HALLWAY) && self.is_empty(RIGHT_HALLWAY) => Some(6+from-DESERT_ROOM_1),
                    MIDDLE_HALLWAY if self.is_empty(RIGHT_HALLWAY) => Some(4+from-DESERT_ROOM_1),                    
                    RIGHT_HALLWAY | INNER_RIGHT_CORNER => Some(2+from-DESERT_ROOM_1),
                    OUTER_RIGHT_CORNER if self.is_empty(INNER_RIGHT_CORNER) => Some(3 + from - DESERT_ROOM_1),
                    _ => None,
                }
            },
            _ => None
        }
    }

    fn perform_move(&mut self, from: usize, to: usize) {
        // let mut grid = self.grid.clone();
        if self.grid[to] != Tile::Empty {
            println!("{}", self);
            println!("moving from {} to {}", from, to);
        }
        assert!(self.grid[to] == Tile::Empty);
        self.grid[to] = self.grid[from];
        self.grid[from] = Tile::Empty;
    }

    fn moves(&self) -> Vec<(GameState, usize)> {
        // If an amphipod can move to its final room, that is always the correct move.
        let mut v = Vec::new();
        for index in (0..23).filter(|&i| !self.is_happy(i)) {
            if let Tile::Occupied(amphipod) = self.grid[index] {                
                let in_a_room = GameState::tile_is_in_a_room(index);
                if in_a_room && GameState::tile_is_in_a_room(index-1) && self.grid[index-1] != Tile::Empty {
                    continue; // Something is blocking the exit
                }
                // println!("Evaluating {} / {}", index, amphipod);
                if index == OUTER_LEFT_CORNER && self.grid[INNER_LEFT_CORNER] != Tile::Empty { 
                    continue;
                }
                if index == OUTER_RIGHT_CORNER && self.grid[INNER_RIGHT_CORNER] != Tile::Empty {
                    continue;
                }
    
                if !in_a_room {
                    let destination_room = match amphipod {
                        Amphipod::Amber => AMBER_ROOM_4,
                        Amphipod::Bronze => BRONZE_ROOM_4,
                        Amphipod::Copper => COPPER_ROOM_4,
                        Amphipod::Desert => DESERT_ROOM_4,
                    };
                    // If there are other types in the destination room, disallow.
                    // But the path is free to the destination room, discard other moves 
                    if ((destination_room-3)..=destination_room).all(|s| self.grid[s] == Tile::Empty || self.is_happy(s)) {
                        let target = ((destination_room-3)..=destination_room).rev().find(|&i| self.is_empty(i)).unwrap();
                        if let Some(cost) = self.free_path(index, target) {
                            let mut n = self.clone();
                            n.perform_move(index, target);
                            //v.push((n, cost * amphipod.energy_per_step()));
                            return vec![(n, cost * amphipod.energy_per_step())];
                        }                
                    }
                } else {
                    v.extend([OUTER_LEFT_CORNER, INNER_LEFT_CORNER, LEFT_HALLWAY, MIDDLE_HALLWAY, RIGHT_HALLWAY, INNER_RIGHT_CORNER, OUTER_RIGHT_CORNER]
                            .iter()
                            .filter(|&&i| self.is_empty(i))
                            .filter_map(|&destination| {

                                if let Some(cost) = self.free_path(index, destination) {
                                    let mut n = self.clone();
                                    n.perform_move(index, destination);
                                    Some((n, cost * amphipod.energy_per_step()))
                                } else {
                                    None
                                }
                            }))
                }
            }
        }
        v
    }

}

#[derive(Clone)]
struct Path {
    states: Vec<GameState>,
    cost: usize
}

pub fn solve(input: &str) -> Solution {
    let m1 = 10321; // Solved by hand

    let s: GameState = input.parse().unwrap();

    let mut visited: HashMap<GameState,usize> = HashMap::new();
    visited.insert(s.clone(), 0);

    let mut paths = vec![Path { states: vec![s], cost: 0 }];
    let destination = GameState::destination();

    while paths.len() > 0 {
        let mut next_paths = Vec::new();
        for path in paths.into_iter() {
            let last = path.states[path.states.len()-1];
            // Get possible Gamestates
            for m in last.moves().into_iter() {
                if let Some(cost) = visited.get(&m.0) {
                    if *cost <= path.cost + m.1 {
                        continue
                    }
                }
                *visited.entry(m.0).or_insert(0) = path.cost + m.1;
                let mut successor = path.states.clone();
                successor.push(m.0);
                let p = Path { states: successor, cost: path.cost + m.1 };
                next_paths.push(p);
            }
        }
        paths = next_paths;
        paths.sort_by_key(|i| i.cost);
    }

    let m2 = visited[&destination];
    // 148923 too high 148123 58923
    // Vec<Gamestate>
    // Sort by lowest cost

    Solution::new(m1,m2)
}

impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
        match self {
            Self::Amber => 'A',
            Self::Bronze => 'B',
            Self::Copper => 'C',
            Self::Desert => 'D',
        })?;
        Ok(())
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => { write!(f, ".")? },
            Self::Occupied(a) => { write!(f, "{}", a)? },
        };
        Ok(())
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#############")?;
        writeln!(f, "#{}{}.{}.{}.{}.{}{}#", self.grid[0], self.grid[1], self.grid[6], self.grid[0xb], self.grid[0x10], self.grid[0x15], self.grid[0x16])?;
        writeln!(f, "###{}#{}#{}#{}###", self.grid[2], self.grid[7], self.grid[0xc], self.grid[0x11])?;        
        writeln!(f, "  #{}#{}#{}#{}#  ", self.grid[3], self.grid[8], self.grid[0xd], self.grid[0x12])?;        
        writeln!(f, "  #{}#{}#{}#{}#  ", self.grid[4], self.grid[9], self.grid[0xe], self.grid[0x13])?;        
        writeln!(f, "  #{}#{}#{}#{}#  ", self.grid[5], self.grid[0xa], self.grid[0xf], self.grid[0x14])?;        
        writeln!(f, "  #########  ")?;        
        Ok(())
    }
}

impl FromStr for Amphipod {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Amphipod::Amber),
            "B" => Ok(Amphipod::Bronze),
            "C" => Ok(Amphipod::Copper),
            "D" => Ok(Amphipod::Desert),
            _ => Err("Unrecognized amphipod"),
        }
    }
}

impl FromStr for Tile {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Empty),
            x => match x.parse::<Amphipod>() {
                Ok(a) => Ok(Tile::Occupied(a)),
                Err(s) => Err(s),
            }
        }
    }
}

impl FromStr for GameState {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amphipods: Vec<Tile> =
        s.lines().skip(2).next().unwrap().split('#').skip(3).take(4).chain(
            s.lines().skip(3).next().unwrap().split('#').skip(1).take(4).chain(
                s.lines().skip(4).next().unwrap().split('#').skip(1).take(4).chain(
                    s.lines().skip(5).next().unwrap().split('#').skip(1).take(4))))
            .flat_map(|x| x.parse::<Tile>()).collect();
        
        if amphipods.len() < 16 {
            Err("Parse error: not enough amphipods")
        } else {        
            Ok(GameState { grid: [
                Tile::Empty, Tile::Empty,
                amphipods[0], amphipods[4], amphipods[8], amphipods[12],
                Tile::Empty,
                amphipods[1], amphipods[5], amphipods[9], amphipods[13],
                Tile::Empty,
                amphipods[2], amphipods[6], amphipods[10], amphipods[14],
                Tile::Empty,
                amphipods[3], amphipods[7], amphipods[11], amphipods[15],
                Tile::Empty, Tile::Empty
            ]})
        }

    }
}