// https://adventofcode.com/2022/day/14
use crate::common::Solution;

const X_OFFSET: usize = 468;
const HEIGHT: usize = 170;
const WIDTH: usize = 87;

const START_X: usize = 500 - X_OFFSET;
const START_Y: usize = 0;

const AIR: u8 = 0;
const SAND: u8 = 1;
const ROCK: u8 = 2;

struct SandPourer {
    cave: [u8; WIDTH*HEIGHT],
    left_spill_point: Option<usize>,
    right_spill_point: Option<usize>,
    added_sand: usize,
}

impl SandPourer {

    fn new(cave: [u8; WIDTH*HEIGHT]) -> SandPourer { SandPourer { cave, left_spill_point: None, right_spill_point: None, added_sand: 0 }}

    fn add_sand_until_full(&mut self, x: usize, y: usize) {
        if self.cave[(y+1)*WIDTH + x] == AIR {
            self.add_sand_until_full(x, y+1);
        }
        if x > 0 {
            if self.cave[(y+1)*WIDTH + x - 1] == AIR {
                self.add_sand_until_full(x-1, y+1);
            }
        } else { 
            self.left_spill_point = Some(if let Some(previous) = self.left_spill_point {
                previous.min(y+1)
            } else { 
                y+1
            });
        }
        if x < WIDTH - 1 {
            if self.cave[(y+1)*WIDTH + x + 1] == AIR {
                self.add_sand_until_full(x+1, y+1);
            }    
        } else {
            self.right_spill_point = Some(if let Some(previous) = self.right_spill_point {
                previous.min(y+1)
            } else { 
                y+1
            });
        }
        self.cave[y*WIDTH + x] = SAND;
        self.added_sand += 1;
    }

    fn total_sand_added(&self, bottom: usize) -> usize {
        self.left_spill_point.map(|y| (bottom - y) * ((bottom - y) + 1) / 2).unwrap_or(0) +
        self.right_spill_point.map(|y| (bottom - y) * ((bottom - y) + 1) / 2).unwrap_or(0) +
        self.added_sand
    }

    fn add_single_grain(&mut self, mut x: usize, mut y: usize) -> bool {
        loop { 
            y += 1;
            if y == HEIGHT {
                return false;
            }
            
            if self.cave[y*WIDTH + x] != AIR {
                if self.cave[y*WIDTH + x - 1] != AIR {
                    if self.cave[y*WIDTH + x + 1] != AIR {
                        self.cave[(y-1)*WIDTH + x] = SAND;
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
    
    fn add_sand_until_it_falls_off(&mut self) -> usize {
        while self.add_single_grain(START_X, START_Y) {
            self.added_sand += 1;
        }
        self.added_sand
    }

    fn add_rock_path_to_cave(&mut self, path: &[(usize,usize)]) {
        for row in path[0].1.min(path[1].1)..=path[0].1.max(path[1].1) {
            for column in path[0].0.min(path[1].0)..=path[0].0.max(path[1].0) {
                self.cave[row*WIDTH + column] = ROCK;
            }
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let mut pourer = SandPourer::new([AIR; WIDTH*HEIGHT]);

    let mut bottom = 0;
    for line in input.lines() {
        let rock_paths: Vec<(usize,usize)> = line.split(" -> ").filter_map(|g| g
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

        bottom = bottom.max(rock_paths.iter().map(|g| g.1).max().unwrap());

        for rock_path in rock_paths.windows(2) { 
            pourer.add_rock_path_to_cave(rock_path) 
        }
    }

    let p1 = pourer.add_sand_until_it_falls_off();

    pourer.add_rock_path_to_cave(&[(0,bottom + 2), (WIDTH - 1,bottom + 2)]);
    pourer.add_sand_until_full(START_X, START_Y);

    let p2 = pourer.total_sand_added(bottom + 2);

    Solution::new(p1,p2)
}
