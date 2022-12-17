// https://adventofcode.com/2022/day/17

use std::collections::HashMap;
use std::hash::Hash;
use crate::common::Solution;

enum Draft {
    Left,
    Right,
}

const NUM_P1_ITERATIONS: usize = 2022;
const WIDTH: u32 = 7;
const MAX_HEIGHT: usize = 8000;
const NUM_ITERATIONS: usize = 1000000000000;
const NUM_FLOORS_TO_HASH: usize = 18;

type Grid = [u8;MAX_HEIGHT];

const ROCK_A: u32 = 0b0000000_0000000_0000000_1111000;
const ROCK_B: u32 = 0b0000000_0100000_1110000_0100000;
const ROCK_C: u32 = 0b0000000_0010000_0010000_1110000;
const ROCK_D: u32 = 0b1000000_1000000_1000000_1000000;
const ROCK_E: u32 = 0b0000000_0000000_1100000_1100000;

fn does_rock_fit(rock: &u32, grid: &Grid, x0: usize, y0: usize) -> bool {
    // x0, y0 is the lower left corner of the rock.
    ((((rock >> WIDTH*0) & 0b1111000) >> x0) as u8) & grid[y0+0] == 0 &&
    ((((rock >> WIDTH*1) & 0b1111000) >> x0) as u8) & grid[y0+1] == 0 &&
    ((((rock >> WIDTH*2) & 0b1111000) >> x0) as u8) & grid[y0+2] == 0 &&
    ((((rock >> WIDTH*3) & 0b1111000) >> x0) as u8) & grid[y0+3] == 0
}

fn paint_rock(rock: &u32, grid: &mut Grid, x0: usize, y0: usize) {
    // x0, y0 is the lower left corner of the rock.
    grid[y0+0] |= (((rock >> WIDTH*0) & 0b1111000) >> x0) as u8;
    grid[y0+1] |= (((rock >> WIDTH*1) & 0b1111000) >> x0) as u8;
    grid[y0+2] |= (((rock >> WIDTH*2) & 0b1111000) >> x0) as u8;
    grid[y0+3] |= (((rock >> WIDTH*3) & 0b1111000) >> x0) as u8;
}

fn hash_top_18_rows(grid_part: &[u8]) -> u128 {
    (0..NUM_FLOORS_TO_HASH).fold(0u128, |v,i| v | (grid_part[i] as u128) << (8*i as u128))
}

#[derive(Hash,PartialEq, Eq)]
struct Configuration {
    rock: usize,
    draft: usize,
    grid: u128
}

pub fn solve(input: &str) -> Solution {
    let mut drafts = input
        .chars()
        .filter_map(|c| match c {
            '>' => Some(Draft::Right),
            '<' => Some(Draft::Left),
            _ => None,
        })
        .enumerate()
        .cycle();

    let rocks = [((4,1),ROCK_A), ((3,3),ROCK_B), ((3,3),ROCK_C), ((1,4),ROCK_D), ((2,2),ROCK_E)];

    let mut grid: Grid = [0u8;MAX_HEIGHT];
    let mut floor = 0;
    let mut seen_configurations: HashMap<Configuration,(usize,usize)> = HashMap::new();
    let mut p1: Option<usize> = None;
    let mut p2: Option<usize> = None;
    for (_iteration, (rock_index,((width,height),rock))) in (1..=NUM_ITERATIONS).zip(rocks.iter().enumerate().cycle()) {
        let mut y = floor + 3;
        let mut x: i32 = 2;
        let draft_index = loop {
            let (draft_index,draft) = drafts.next().unwrap();
            match draft {
                Draft::Left => if x > 0 && does_rock_fit(rock, &grid, (x - 1) as usize, y as usize) { x -= 1; },
                Draft::Right => if x + width < WIDTH as i32 && does_rock_fit(rock, &grid, (x + 1) as usize, y as usize) { x += 1},
            }
            if y == 0 { break draft_index; } // Lands on bottom.
            if does_rock_fit(rock, &grid, x as usize, (y - 1) as usize) {
                y -= 1;
            } else {
                break draft_index;
            }
        };
        paint_rock(rock, &mut grid, x as usize, y as usize);

        floor = floor.max(y + height);

        // The iteration period must be a multiple of 5, so the iteration where 
        // 1000000000000 % iteration_period == iteration_index % iteration_period 
        // must also be a multiple of 5 (1000000000000 is of course a multiple of 5).
        if (_iteration % 5 == 0) && floor > NUM_FLOORS_TO_HASH {
            let start = floor - NUM_FLOORS_TO_HASH + 1;
            let configuration = Configuration {
                rock: rock_index,
                draft: draft_index,
                grid: hash_top_18_rows(&grid[start..(start + NUM_FLOORS_TO_HASH)])
            };
            if let Some((iteration, floor_height)) = seen_configurations.get(&configuration) {
                let iteration_period = _iteration - iteration;
                if NUM_ITERATIONS % iteration_period == _iteration % iteration_period {
                    let floor_period = floor - floor_height;
                    let num_remaining = (NUM_ITERATIONS - _iteration) / iteration_period;
                    p2 = Some(floor + num_remaining * floor_period);
                } 
            } else {
                seen_configurations.insert(configuration, (_iteration, floor));
            }
        }

        if _iteration == NUM_P1_ITERATIONS {
            p1 = Some(floor);
        }
        if _iteration > NUM_P1_ITERATIONS && p2.is_some() {
            break;
        }
    }
    
    Solution::new(p1.unwrap(),p2.unwrap())
}
