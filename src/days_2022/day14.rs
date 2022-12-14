// https://adventofcode.com/2022/day/14
use crate::common::Solution;

const X_OFFSET: usize = 468;
const HEIGHT: usize = 170;
const WIDTH: usize = 87;

const START_X: usize = 500 - X_OFFSET;
const START_Y: usize = 0;

type Cave = [u8; WIDTH*HEIGHT];
const AIR: u8 = 0;
const SAND: u8 = 1;
const ROCK: u8 = 2;

#[derive(Debug)]
struct SandAddingResult {
    num_sand: usize,
    highest_spill_on_left_side: Option<usize>,
    highest_spill_on_right_side: Option<usize>,
}

impl SandAddingResult {
    fn new() -> SandAddingResult { SandAddingResult { num_sand: 0, highest_spill_on_left_side: None, highest_spill_on_right_side: None }}

    fn update(&mut self, other: SandAddingResult) {
        self.num_sand += other.num_sand;
        if let Some(spill) = other.highest_spill_on_left_side {
            self.highest_spill_on_left_side = Some(spill.min(self.highest_spill_on_left_side.unwrap_or(usize::MAX)));
        }
        if let Some(spill) = other.highest_spill_on_right_side {
            self.highest_spill_on_right_side = Some(spill.min(self.highest_spill_on_right_side.unwrap_or(usize::MAX)));
        }
    }

    fn spill_on_left_side(&mut self, y: usize) {
        self.highest_spill_on_left_side = Some(y.max(self.highest_spill_on_left_side.unwrap_or(0)));
    }

    fn spill_on_right_side(&mut self, y: usize) {
        self.highest_spill_on_right_side = Some(y.max(self.highest_spill_on_right_side.unwrap_or(0)));
    }

    fn num_spilled_in_relation_to_bottom_rock(&self, bottom: usize) -> usize {
        self.highest_spill_on_left_side.map(|y| (bottom - y) * ((bottom - y) + 1) / 2).unwrap_or(0) +
        self.highest_spill_on_right_side.map(|y| (bottom - y) * ((bottom - y) + 1) / 2).unwrap_or(0)
    }

}

fn add_sand_until_full(cave: &mut Cave, x: usize, y: usize) -> SandAddingResult {
    let mut result = SandAddingResult::new();
    if cave[(y+1)*WIDTH + x] == AIR {
        result.update(add_sand_until_full(cave, x, y+1));
    }
    if x > 0 {
        if cave[(y+1)*WIDTH + x - 1] == AIR {
            result.update(add_sand_until_full(cave, x-1, y+1));
        }
    } else { 
        result.spill_on_left_side(y+1);
    }
    if x < WIDTH - 1 {
        if cave[(y+1)*WIDTH + x + 1] == AIR {
            result.update(add_sand_until_full(cave, x+1, y+1));
        }    
    } else {
        result.spill_on_right_side(y + 1);
    }
    cave[y*WIDTH + x] = SAND;
    result.num_sand += 1;
    result
}

fn add_single_grain(cave: &mut Cave, mut x: usize, mut y: usize) -> bool {
    loop { 
        y += 1;
        if y == HEIGHT {
            return false;
        }
        
        if cave[y*WIDTH + x] != AIR {
            if cave[y*WIDTH + x - 1] != AIR {
                if cave[y*WIDTH + x + 1] != AIR {
                    cave[(y-1)*WIDTH + x] = SAND;
                    return true
                } else {
                    x += 1;
                }
            } else {
                x -= 1;
            }
        }
    }
}

fn add_sand_until_it_falls_off(cave: &mut [u8; WIDTH*HEIGHT]) -> usize {
    let mut p1 = 0;
    while add_single_grain(cave, START_X, START_Y) {
        p1 += 1;
    }
    p1
}

fn add_rock_path(cave: &mut Cave, path: &[(usize,usize)]) {
    for row in path[0].1.min(path[1].1)..=path[0].1.max(path[1].1) {
        for column in path[0].0.min(path[1].0)..=path[0].0.max(path[1].0) {
            cave[row*WIDTH + column] = ROCK;
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let mut cave: Cave = [AIR; WIDTH*HEIGHT];

    let mut bottom = 0;
    for line in input.lines() {
        let groups: Vec<(usize,usize)> = line.split(" -> ").filter_map(|g| g
                .split_once(",")
                .map(|(s1,s2)| {
                    let n1 = s1.parse::<usize>().unwrap();
                    let n2 = s2.parse::<usize>().unwrap();
                    assert!(n1 >= X_OFFSET, "Column too far to the left");
                    assert!(n1 < X_OFFSET + WIDTH, "Too narrow");
                    assert!(n2 < HEIGHT, "Too low");
                    (n1 - X_OFFSET, n2)
                }))    
                .collect();

        bottom = bottom.max(groups.iter().map(|g| g.1).max().unwrap());

        for g in groups.windows(2) { 
            add_rock_path(&mut cave, g) 
        }
    }

    let p1 = add_sand_until_it_falls_off(&mut cave);

    add_rock_path(&mut cave, &[(0,bottom + 2), (WIDTH - 1,bottom + 2)]);

    let r = add_sand_until_full(&mut cave, START_X, START_Y);
    let p2 = p1 + r.num_sand + r.num_spilled_in_relation_to_bottom_rock(bottom + 2);

    Solution::new(p1,p2)
}
