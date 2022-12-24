// https://adventofcode.com/2022/day/23

use crate::common::Solution;
use pathfinding::prelude::bfs;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
const RIGHT: u8 = '>' as u8;
const LEFT: u8 = '<' as u8;
const UP: u8 = '^' as u8;
const DOWN: u8 = 'v' as u8;
const EMPTY: u8 = '.' as u8;
const WALL: u8 = '#' as u8;

#[derive(Debug)]
struct Blizzard {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug,Clone,Eq)]
struct State {
    x: u8,
    y: u8,
    turn: u16,
}

impl State {
    fn wait(&self) -> State {
        State { x: self.x, y: self.y, turn: self.turn + 1 }
    }

    fn go(&self, direction: &Direction) -> State {
        match direction {
            Direction::Down => State { x: self.x, y: self.y + 1, turn: self.turn + 1 },
            Direction::Up => State { x: self.x, y: self.y - 1, turn: self.turn + 1 },
            Direction::Left => State { x: self.x - 1, y: self.y, turn: self.turn + 1 },
            Direction::Right => State { x: self.x + 1, y: self.y, turn: self.turn + 1 },
        }
    } 

    fn is_end(&self) -> bool { self.x == (WIDTH - 2) as u8 && self.y == (HEIGHT - 2) as u8 }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && 
        (self.turn % (NUM_CONFIGURATIONS as u16)) == 
        (other.turn % (NUM_CONFIGURATIONS as u16))
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        (self.turn % (NUM_CONFIGURATIONS as u16)).hash(state);
    }
}

const WIDTH: usize = 120 + 2;
const HEIGHT: usize = 25 + 4;
const NUM_CONFIGURATIONS: usize = 600;

pub fn solve(input: &str) -> Solution {
    let mut blizzards: Vec<Blizzard> = vec![];
    
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.chars().enumerate() {
            if let Some(d) = match b {
                '<' => Some(Direction::Left),
                '>' => Some(Direction::Right),
                '^' => Some(Direction::Up),
                'v' => Some(Direction::Down),
                '.' | '#' => None,
                _ => panic!("?"), } { 
                blizzards.push(Blizzard { x, y: y + 1, direction: d });
            }
        }
    }

    let mut grid = [[WALL;WIDTH*HEIGHT]; NUM_CONFIGURATIONS];

    for c in 0..NUM_CONFIGURATIONS {
        for y in 2..(HEIGHT - 2) {
            for x in 1..(WIDTH - 1) {
                grid[c][y*WIDTH + x] = EMPTY;
            }
        }
        
        grid[c][1*WIDTH + 1] = EMPTY;
        grid[c][(HEIGHT - 2)*WIDTH + WIDTH - 2] = EMPTY;

        for blizzard in blizzards.iter() {
            grid[c][blizzard.y * WIDTH + blizzard.x] = if grid[c][blizzard.y * WIDTH + blizzard.x] == EMPTY {
                 match blizzard.direction {
                    Direction::Down => DOWN,
                    Direction::Left => LEFT,
                    Direction::Right => RIGHT,
                    Direction::Up => UP,
                }
            } else { 'M' as u8 }
        }

        for blizzard in blizzards.iter_mut() {
            match blizzard.direction {
                Direction::Down => { blizzard.y += 1; if blizzard.y == HEIGHT-2 { blizzard.y = 2; } },
                Direction::Left => { blizzard.x -= 1; if blizzard.x == 0 { blizzard.x = WIDTH - 2; } },
                Direction::Up => { blizzard.y -= 1; if blizzard.y == 1 { blizzard.y = HEIGHT - 3; } },
                Direction::Right => { blizzard.x += 1; if blizzard.x == WIDTH - 1 { blizzard.x = 1; }  },
            };
        }
    }

    let get_successors = |state: &State| {
        let g = grid[((state.turn + 1) as usize) % NUM_CONFIGURATIONS];
        let mut v: Vec<State> = vec![];
        let x = state.x as usize;
        let y = state.y as usize;

        if g[(y + 1)*WIDTH + x] == EMPTY { v.push(state.go(&Direction::Down)); }
        if g[(y - 1)*WIDTH + x] == EMPTY { v.push(state.go(&Direction::Up)); }
        if g[y*WIDTH + x + 1] == EMPTY { v.push(state.go(&Direction::Right)); }
        if g[y*WIDTH + x - 1] == EMPTY { v.push(state.go(&Direction::Left)); }
        if g[y*WIDTH + x] == EMPTY { v.push(state.wait()); }

        v.into_iter()
    };

    let p = bfs(&State { x: 1, y: 1, turn: 0}, get_successors,        
    |state| state.is_end()).unwrap();

    let p2 = bfs(&p.last().cloned().unwrap(), get_successors,        
        |state| state.x == 1 && state.y == 1).unwrap();

    let p3 = bfs(&p2.last().cloned().unwrap(), get_successors,        
        |state| state.is_end()).unwrap();

    let p1 = p.len()-1;
    let p2 = p1 + p2.len()-1 + p3.len()-1;

    Solution::new(p1,p2)
}
