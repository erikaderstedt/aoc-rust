// https://adventofcode.com/2022/day/14
use crate::common::Solution;
use std::ops::Range;

const X_OFFSET: usize = 468;
const HEIGHT: usize = 170;
const WIDTH: usize = 87;

const START_X: usize = 500 - X_OFFSET;
const START_Y: usize = 0;

const AIR: u8 = 0;
const SAND: u8 = 1;
const ROCK: u8 = 2;

type Cave = [u8; WIDTH*HEIGHT];

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

fn add_sand_until_it_falls_off(cave: &mut Cave) -> usize {
    let mut added_sand = 0;
    while add_single_grain(cave, START_X, START_Y) {
        added_sand += 1;
    }
    added_sand
}

fn add_rock_path_to_cave(cave: &mut Cave, path: &[(usize,usize)]) {
    let start = path[0];
    let stop = path[1];
    for row in start.1.min(stop.1)..=start.1.max(stop.1) {
        for column in start.0.min(stop.0)..=start.0.max(stop.0) {
            cave[row*WIDTH + column] = ROCK;
        }
    }
}

fn iterate_rock_ranges(cave: &Cave, row: usize) -> RockRangeIterator { 
    RockRangeIterator { cave, row, index: 0 }
}

struct RockRangeIterator<'a> {
    cave: &'a Cave,
    row: usize,
    index: usize,
}

impl<'a> Iterator for RockRangeIterator<'a> {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut j = (self.index..WIDTH)
            .skip_while(|i| self.cave[self.row * WIDTH + i] != ROCK)
            .take_while(|i| self.cave[self.row * WIDTH + i] == ROCK);
        match j.next() {
            Some(i1) => {
                let i2 = j.last().unwrap_or(i1);
                self.index = i2 + 1;
                Some(Range { start: i1, end: i2 + 1 })
            },
            None => None,
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let mut cave = [AIR; WIDTH*HEIGHT];

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
            add_rock_path_to_cave(&mut cave, rock_path) 
        }
    }

    let p1 = add_sand_until_it_falls_off(&mut cave);

    let mut rock_ranges: Vec<Range<usize>> = Vec::new();
    let mut p2: usize = (bottom + 2) * (bottom + 2);
    for y in 1..(bottom + 2) {
        rock_ranges = rock_ranges.iter_mut()
            .filter(|r| r.len() > 2)
            .map(|r| Range { start: r.start + 1, end: r.end - 1 })
            .chain(iterate_rock_ranges(&cave, y))
            .collect();
        rock_ranges.sort_by_key(|r| r.start);
        if rock_ranges.len() > 1 {
            let mut i = 0;
            while i < rock_ranges.len() - 1 {
                if rock_ranges[i + 1].start <= rock_ranges[i].end {
                    rock_ranges[i].end = rock_ranges[i+1].end.max(rock_ranges[i].end);
                    rock_ranges.remove(i + 1);
                } else {
                    i += 1;
                }
            }
        }
        p2 -= rock_ranges.iter().map(|r| r.len()).sum::<usize>();
    }

    Solution::new(p1,p2)
}
