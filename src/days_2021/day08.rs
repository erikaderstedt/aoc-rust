// https://adventofcode.com/2021/day/7
use crate::common::Solution;

const NUM_SEGMENTS: usize = 7;
const NUM_DISPLAYS: usize = 4;
const SPACE: u8 = ' ' as u8;
const PIPE: u8 = '|' as u8;

fn parse(s: &str) -> ([u8;10],[u8;NUM_DISPLAYS]) {
    let mut samples = [0u8;10];
    let mut displays = [0u8;NUM_DISPLAYS];
    let mut i = 0isize;
    let mut first_part = true;
    for &c in s.as_bytes() {
        if c == SPACE {
            i += 1;
        } else if c == PIPE {
            first_part = false;
            i = -1;
        } else if first_part {
            samples[i as usize] |= 1 << (c - 97);
        } else {
            displays[i as usize] |= 1 << (c - 97);
        }
    }
    (samples,displays)
}

fn interpret_digits(wires: &[u8;10], digits: &[u8;NUM_DISPLAYS] ) -> usize {
    let mut one = 0;
    let mut four = 0;
    for &w in wires {
        match w.count_ones() {
            2 => { one = w; },
            4 => { four = w; },
            _ => { },
        }
    }
    let mut digitmap = [0; 1 << NUM_SEGMENTS];
    for &w in wires {
        digitmap[w as usize] = match w.count_ones() {
            2 => 1, 3 => 7, 4 => 4, 7 => 8,
            6 if (w & one).count_ones() == 1 => 6,
            6 if (w & four).count_ones() == 3 => 0,
            6 => 9,
            5 if (w & one).count_ones() == 2 => 3,
            5 if (w & four).count_ones() == 2 => 2,
            5 => 5,
            _ => panic!("Unexpected number of bits in sample signal"),
        };
    }

    digits.iter().fold(0, |s, &digit| { 10*s + digitmap[digit as usize] })
}

pub fn solve(input: &str) -> Solution {
    let displays: Vec<([u8;10],[u8;NUM_DISPLAYS])> =
        input.lines().map(|line| parse(line)).collect();

    let m1 = displays.iter().fold(0, |i, digits| i + digits.1.iter()
        .filter(|display| matches!(display.count_ones(), 2 | 4 | 3 | 7))
        .count());
    
    let m2 = displays.into_iter().fold(0usize, 
        |sum, (wires, digits)| sum + interpret_digits(&wires, &digits));

    Solution::new(m1, m2)
}
