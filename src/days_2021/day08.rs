// https://adventofcode.com/2021/day/7
use crate::common::Solution;

const NUM_SEGMENTS: usize = 7;
// The segments that must be set for a character with N set segments (0-7)
const INTERSECTION: [u8;NUM_SEGMENTS+1] = [
    0,0,0b0100100,0b0100101,0b0101110, 0b1001001, 0b1100011, 0b1111111];
const DIGITS: [usize;128] = {                                       
    let mut digits_lookup = [0; 128];                               //  0000
    digits_lookup[0b1110111] = 0;    digits_lookup[0b0100100] = 1;  // 1    2   
    digits_lookup[0b1011101] = 2;    digits_lookup[0b1101101] = 3;  // 1    2 
    digits_lookup[0b0101110] = 4;    digits_lookup[0b1101011] = 5;  //  3333
    digits_lookup[0b1111011] = 6;    digits_lookup[0b0100101] = 7;  // 4    5
    digits_lookup[0b1111111] = 8;    digits_lookup[0b1101111] = 9;  // 4    5
    digits_lookup                                                   //  6666
};

fn segments(s: &str) -> u8 {
    s.chars().fold(0u8, |x, c| x + (1 << ((c as u8)-('a' as u8))))
}

fn interpret_digits(wires: &Vec<u8>, digits: &Vec<u8> ) -> usize {
    let mut possibilities = [127u8; NUM_SEGMENTS]; // Each segment can be any bit
    for w in wires {
        let len = w.count_ones() as usize;
        for i in 0..NUM_SEGMENTS { 
            if INTERSECTION[len] & (1 << i) > 0 { possibilities[i] &= w }
        }
    }    
    let mut wire_names = [0u8; NUM_SEGMENTS];
    while possibilities.iter().any(|p| *p > 0) {
        let mut found: u8 = 0;                
        for (i, p) in possibilities.iter().enumerate() {
            if p.count_ones() == 1 {
                wire_names[p.trailing_zeros() as usize] = i as u8;
                found |= p.clone();
            }
        }
       for po in possibilities.iter_mut() { *po &= !found }
    }
    digits.iter().fold(0, |s, digit| {
        let mut x = 0u8;
        for i in 0..NUM_SEGMENTS { 
            if digit & (1 << i) > 0 { x |= 1 << wire_names[i] } 
        }
        10 * s + DIGITS[x as usize]
    })
}

pub fn solve(input: &str) -> Solution {
    let displays: Vec<(Vec<u8>,Vec<u8>)> =
        input.lines().map(|line| {
            let (f,s) = line.split_once(" | ").unwrap();
            (f.split(' ').map(|j| segments(j)).collect(), 
            s.split(' ').map(|j| segments(j)).collect())
        }).collect();

    let m1 = displays.iter().fold(0, |i, digits| i + digits.1.iter()
        .filter(|display| match display.count_ones() { 2 | 4 | 3 | 7 => true, _ => false })
        .count());

    let m2 = displays.into_iter().fold(0usize, 
        |sum, (wires, digits)| sum + interpret_digits(&wires, &digits));

    Solution::new(m1, m2)
}
