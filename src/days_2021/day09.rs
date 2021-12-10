// https://adventofcode.com/2021/day/9
use crate::common::Solution;

const COLS: usize = 100;
const ROWS: usize = 100;
const MOUNTAIN: u8 = b'9';
const WIDTH: usize = ROWS + 2;
const HEIGHT: usize = COLS + 2;
type HeightMap = [u8;WIDTH*HEIGHT];

fn size_of_basin_at(index: usize, heights: &HeightMap) -> usize {
    let mut members: Vec<usize> = Vec::new();
    
    add(index, &mut members, heights);
    members.len()
}

fn add(index: usize, points_in_basin: &mut Vec<usize>, heights: &HeightMap) {
    points_in_basin.push(index);

    if heights[index-WIDTH] < MOUNTAIN && !points_in_basin.contains(&(index-WIDTH)) { 
        add(index-WIDTH, points_in_basin, heights);
    }
    if heights[index+WIDTH] < MOUNTAIN && !points_in_basin.contains(&(index+WIDTH)) { 
        add(index+WIDTH, points_in_basin, heights);
    }
    if heights[index-1] < MOUNTAIN && !points_in_basin.contains(&(index-1)) { 
        add(index-1, points_in_basin, heights);
    }
    if heights[index+1] < MOUNTAIN && !points_in_basin.contains(&(index+1)) { 
        add(index+1, points_in_basin, heights);
    }
}

pub fn solve(input: &str) -> Solution {
    let mut height_map = [MOUNTAIN;WIDTH*HEIGHT];
    for (y, line) in input.lines().enumerate() {
        height_map[(y+1)*WIDTH+1..(y+1)*WIDTH+COLS+1].copy_from_slice(line.as_bytes());
    }
    let mut m1 = 0usize;
    let mut basin_sizes = Vec::new();
    for y in 1..ROWS+1 {
        for x in 1..COLS+1 {
            let th = height_map[y*WIDTH+x];
            if th < height_map[y*WIDTH+x-1] && th < height_map[y*WIDTH+x+1] &&
                th < height_map[(y-1)*WIDTH+x] &&  th < height_map[(y+1)*WIDTH+x] {
                m1 += 1 + ((th - b'0') as usize);
                basin_sizes.push(size_of_basin_at(y*WIDTH+x, &height_map));
            }

        }
    }
    basin_sizes.sort();
    let m2: usize = basin_sizes.into_iter().rev().take(3).product();
    Solution::new(m1, m2)
}
