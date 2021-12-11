// https://adventofcode.com/2021/day/11
use crate::common::Solution;

const SIZE: usize = 10;
const ALL: usize = SIZE*SIZE;

pub fn solve(input: &str) -> Solution {
    let mut octopuses = [0u32;SIZE*SIZE];
    for (y, line) in input.lines().enumerate() {
        for (x, &c) in line.as_bytes().iter().enumerate() {
            octopuses[y*SIZE+x] = (c as u32) - 47;
        }
    }

    let mut m1 = 0;
    let m2: usize;
    let mut step = 1; 
    loop {
        let mut flashed = [false;SIZE*SIZE];
        while match octopuses.iter().enumerate().position(|(i,o)| !flashed[i] && *o > 9) {
            Some(position) => {
                let not_left_edge = position%SIZE > 0;
                let not_right_edge = position%SIZE < SIZE - 1;
                let not_top_edge = position >= SIZE;
                let not_bottom_edge = position < ALL - SIZE;
                if not_left_edge { octopuses[position - 1] += 1; }
                if not_left_edge && not_top_edge { octopuses[position - SIZE - 1] += 1; }
                if not_left_edge && not_bottom_edge { octopuses[position + SIZE - 1] += 1; }
                if not_top_edge { octopuses[position - SIZE] += 1; }
                if not_bottom_edge { octopuses[position + SIZE] += 1; }
                if not_right_edge { octopuses[position + 1] += 1; }
                if not_right_edge && not_top_edge { octopuses[position - SIZE + 1] += 1; }
                if not_right_edge && not_bottom_edge { octopuses[position + SIZE + 1] += 1; }
                flashed[position] = true;
                if step <= 100 { m1 += 1; }
                true
            },
            None => false,
        } {}
        if flashed.iter().all(|f| *f) {
            m2 = step;
            break;
        }
        // Set all flashed to 0
        octopuses.iter_mut().for_each(|o| if *o > 9 { *o = 1 } else { *o = *o + 1 });
        step += 1;
    }
    
    Solution::new(m1, m2)
}
