// https://adventofcode.com/2022/day/17

use std::collections::HashMap;
use std::hash::Hash;
use crate::common::Solution;

enum Draft {
    Left,
    Right,
}

const NUM_P1_ITERATIONS: usize = 2022;
const WIDTH: usize = 7;
const MAX_HEIGHT: usize = 8000;
type Rock = [u8;16];
type Grid = [u8;WIDTH*MAX_HEIGHT];

const ROCK_A: Rock = [1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0];
const ROCK_B: Rock = [0,1,0,0,1,1,1,0,0,1,0,0,0,0,0,0];
const ROCK_C: Rock = [1,1,1,0,0,0,1,0,0,0,1,0,0,0,0,0];
const ROCK_D: Rock = [1,0,0,0,1,0,0,0,1,0,0,0,1,0,0,0];
const ROCK_E: Rock = [1,1,0,0,1,1,0,0,0,0,0,0,0,0,0,0];

fn does_rock_fit(rock: &Rock, grid: &Grid, x0: usize, y0: usize) -> bool {
    // x0, y0 is the lower left corner of the rock.
    (0..16).all(|i| {
        let x = i % 4;
        let y = i / 4;
        rock[i] == 0 || grid[(y0+y)*WIDTH + (x0 + x)] == 0
    })
}

fn paint_rock(rock: &Rock, grid: &mut Grid, x0: usize, y0: usize) {
    // x0, y0 is the lower left corner of the rock.
    for i in 0..16 {
        let x = i % 4;
        let y = i / 4;
        if rock[i] == 1 {
            grid[(y0+y)*WIDTH + (x0 + x)] = 1;
        }
    }
}

fn hash_top_18_rows(grid_part: &[u8]) -> u128 {
    (0..(18*WIDTH)).fold(0u128, |v,i| if grid_part[i] == 1 { v + (1u128 << i) } else { v })
}

#[derive(Hash,PartialEq, Eq)]
struct Configuration {
    rock: usize,
    draft: usize,
    grid: u128
}

pub fn solve(input: &str) -> Solution {

    let drafts: Vec<Draft> = input.chars().filter_map(|c| match c {
        '>' => Some(Draft::Right),
        '<' => Some(Draft::Left),
        _ => None,
    }).collect();

    let rocks = [((4,1),ROCK_A), ((3,3),ROCK_B), ((3,3),ROCK_C), ((1,4),ROCK_D), ((2,2),ROCK_E)];

    let mut grid: Grid = [0u8;WIDTH*MAX_HEIGHT];
    let mut draft_source = drafts.iter().enumerate().cycle();
    let mut floor = 0;
    let mut seen_configurations: HashMap<Configuration,(usize,usize)> = HashMap::new();
    let mut p1: Option<usize> = None;
    let mut p2: Option<usize> = None;
    for (_iteration, (rock_index,((width,height),rock))) in (0..1000000000000).zip(rocks.iter().enumerate().cycle()) {
        let mut y = floor + 3;
        let mut x: i32 = 2;
        let draft_index = loop {
            let (draft_index,draft) = draft_source.next().unwrap();
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

        if floor > 18 {
            let start = floor - 17;
            let configuration = Configuration {
                rock: rock_index,
                draft: draft_index,
                grid: hash_top_18_rows(&grid[start*WIDTH .. (start + 18)*WIDTH])
            };
            if let Some((iteration, floor_height)) = seen_configurations.get(&configuration) {
                let iteration_period = _iteration - iteration;
                if 1000000000000 % iteration_period == _iteration % iteration_period {
                    let floor_period = floor - floor_height;
                    let num_remaining = (1000000000000 - _iteration) / iteration_period;
                    p2 = Some(floor + num_remaining * floor_period - 1);
                } 
            } else {
                seen_configurations.insert(configuration, (_iteration, floor));
            }
        }

        if _iteration == NUM_P1_ITERATIONS - 1 {
            p1 = Some(floor);
        }
        if _iteration > NUM_P1_ITERATIONS && p2.is_some() {
            break;
        }
    }
    
    Solution::new(p1.unwrap(),p2.unwrap())
}
