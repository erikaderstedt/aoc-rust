// https://adventofcode.com/2021/day/11
use crate::common::Solution;

const SIZE: usize = 10;
const ALL: usize = SIZE*SIZE;

fn step(energy_levels: &mut [u32;ALL]) -> usize {
    let mut not_flashed = [true;ALL];
    while match energy_levels.iter().enumerate().position(|(i, o)| not_flashed[i] && *o > 9) {
        Some(position) => {
            let not_left_edge = position%SIZE > 0;
            let not_right_edge = position%SIZE < SIZE - 1;
            let not_top_edge = position >= SIZE;
            let not_bottom_edge = position < ALL - SIZE;
            if not_left_edge { energy_levels[position - 1] += 1; }
            if not_left_edge && not_top_edge { energy_levels[position - SIZE - 1] += 1; }
            if not_left_edge && not_bottom_edge { energy_levels[position + SIZE - 1] += 1; }
            if not_top_edge { energy_levels[position - SIZE] += 1; }
            if not_bottom_edge { energy_levels[position + SIZE] += 1; }
            if not_right_edge { energy_levels[position + 1] += 1; }
            if not_right_edge && not_top_edge { energy_levels[position - SIZE + 1] += 1; }
            if not_right_edge && not_bottom_edge { energy_levels[position + SIZE + 1] += 1; }
            not_flashed[position] = false;
            true
        },
        None => false,
    } {}
    energy_levels.iter_mut().for_each(|o| if *o > 9 { *o = 1 } else { *o += 1 });
    ALL - not_flashed.iter().filter(|f| **f).count()
}

pub fn solve(input: &str) -> Solution {
    let mut energy_levels = [0u32;ALL];
    for (y, line) in input.lines().enumerate() {
        for (x, &c) in line.as_bytes().iter().enumerate() {
            energy_levels[y*SIZE+x] = (c as u32) - 47; // Start at 1
        }
    }

    let m1 = (0..100).fold(0, |s, _| s + step(&mut energy_levels));
    let m2 = (101..).find(|_| step(&mut energy_levels) == ALL).unwrap();
    
    Solution::new(m1, m2)
}
