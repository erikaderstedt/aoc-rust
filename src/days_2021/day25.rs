// https://adventofcode.com/2021/day/25
use crate::common::Solution;
const WIDTH: usize = 139;
const HEIGHT: usize = 137;

const EAST: u8 = b'>';
const SOUTH: u8 = b'v';
const EMPTY: u8 = b'.';

pub fn solve(input: &str) -> Solution {
    let mut grid = [[b'.';WIDTH];HEIGHT];
    for (y,line) in input.lines().enumerate() {
        let cs = line.as_bytes();
        for i in 0..cs.len() { 
            grid[y][i] = cs[i];
        }
    }

    let mut turn_index = 0;
    let m1 = loop {
        let mut num_changes = 0;

        for y in 0..HEIGHT {
            let mut x = 0;
            let x0 = grid[y][0];
            while x < WIDTH {
                if grid[y][x] == EAST && 
                    ((x == WIDTH - 1 && x0 == EMPTY) || (x < WIDTH - 1 && grid[y][x+1] == EMPTY)) {
                        grid[y][x] = EMPTY;
                        grid[y][(x+1)%WIDTH] = EAST;
                        num_changes += 1;
                        x += 1;
                    }
                
                x += 1;
            }
        }

        // Find south-facing with available space,
        for x in 0..WIDTH {
            let mut y = 0;
            let y0 = grid[0][x];
            while y < HEIGHT {
                if grid[y][x] == SOUTH && 
                    ((y == HEIGHT-1 && y0 == EMPTY) || 
                    (y < HEIGHT - 1 && grid[y+1][x] == EMPTY)) {
                    grid[y][x] = EMPTY;
                    grid[(y + 1) % HEIGHT][x] = SOUTH;
                    num_changes += 1;
                    y += 1;
                }

                y += 1;
            }
        }
        turn_index += 1;
        if num_changes == 0 {
            break turn_index
        }
    };

    Solution::new(m1, 0)
}
