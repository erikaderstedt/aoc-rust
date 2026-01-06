// https://adventofcode.com/2017/day/16

use crate::common::Solution;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
enum Move {
    Spin(u64),
    Exchange(u64, u64),
    Partner(u8, u8),
}

fn swap(state: u64, a: &u64, b: &u64) -> u64 {
    let value_a = (state >> (a * 4)) & 0xf;
    let value_b = (state >> (b * 4)) & 0xf;
    (state ^ (value_a << (a * 4)) ^ (value_b << (b * 4)))
        + (value_a << (b * 4))
        + (value_b << (a * 4))
}

fn rotate_right(state: u64, s: &u64) -> u64 {
    (state << (s * 4)) + (state >> ((16 - s) * 4))
}

fn dance(mut state: u64, moves: &Vec<Move>) -> u64 {
    for m in moves.iter() {
        // println!("{:064b} - {:?}", state, m);
        state = match m {
            Move::Spin(s) => rotate_right(state, s),
            Move::Exchange(a, b) => swap(state, a, b),
            Move::Partner(a, b) => {
                let i = (0u64..16)
                    .find(|i| (state >> (4 * i)) & 0xf == (*a as u64))
                    .unwrap();
                let j = (0u64..16)
                    .find(|i| (state >> (4 * i)) & 0xf == (*b as u64))
                    .unwrap();
                swap(state, &i, &j)
            }
        }
    }
    state
}

fn state_to_string(state: &u64) -> String {
    (0..16)
        .map(|i| (((state >> (4 * i)) & 0xf) as u8 + ('a' as u8)) as char)
        .collect()
}

pub fn solve(input: &str) -> Solution {
    let moves: Vec<Move> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    // 16 characters, 16 positions -> 4 bits, 16 positions.
    // We can represent the state as u64. First position in lowest bits
    // The performance increase was about 10% so not really worth it, but
    // I'm keeping it in.
    let mut state = (0..16).map(|v| v << (v * 4)).reduce(|a, b| a | b).unwrap();

    state = dance(state, &moves);

    let p1: String = state_to_string(&state);

    let mut seen: Vec<u64> = vec![];
    while !seen.contains(&state) {
        seen.push(state);
        state = dance(state, &moves);
    }

    let i = seen.iter().position(|p| *p == state).unwrap();
    let period = seen.len() - i;
    let offset = (1000000000 % period) - 1;
    let p2 = state_to_string(&seen[offset]);

    Solution::new(p1, p2)
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            's' => {
                let n = s[1..].parse::<u64>().map_err(|_| "Invalid spin length")?;
                Ok(Move::Spin(n))
            }
            'x' => {
                let (a, b) = s[1..]
                    .split('/')
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect_tuple()
                    .unwrap();
                Ok(Move::Exchange(a, b))
            }
            'p' => {
                let (a, b) = s
                    .chars()
                    .skip(1)
                    .step_by(2)
                    .map(|c| (c as u8) - ('a' as u8))
                    .collect_tuple()
                    .unwrap();
                Ok(Move::Partner(a, b))
            }
            _ => Err("Invalid move"),
        }
    }
}
