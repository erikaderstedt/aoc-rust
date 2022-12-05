// https://adventofcode.com/2022/day/5

use crate::common::{Solution};
use std::{str::FromStr, collections::VecDeque, str};

const CRATE_WIDTH: usize = 4;

#[derive(Debug)]
struct Instruction {
    num_crates: usize,
    source: usize,
    dest: usize
}

fn top_of_stacks(stacks: &Vec<VecDeque<u8>>) -> String {
    let tops: Vec<u8> = stacks.iter().map(|s| s.back().unwrap().clone()).collect();
    str::from_utf8(&tops[..]).unwrap().to_string()
}

fn simulate_with_cratemover_9000(moves: &Vec<Instruction>, stacks: &Vec<VecDeque<u8>>) -> String {
    let mut stacks = stacks.clone();

    for m in moves.iter() {
        for _ in 0..m.num_crates {
            let top_crate = stacks[m.source].pop_back().unwrap();
            stacks[m.dest].push_back(top_crate);
        }
    }

    top_of_stacks(&stacks)
}

fn simulate_with_cratemover_9001(moves: &Vec<Instruction>, stacks: &mut Vec<VecDeque<u8>>) -> String {
    for m in moves.iter() {
        let num_crates_in_source = stacks[m.source].len();
        let mut p = stacks[m.source].split_off( num_crates_in_source - m.num_crates);
        stacks[m.dest].append(&mut p);
    }

    top_of_stacks(stacks)
}

pub fn solve(input: &str) -> Solution {
    let moves: Vec<Instruction> = input
        .lines()
        .skip_while(|line| !line.starts_with("move"))
        .filter_map(|m| match m.parse::<Instruction>() {
                Ok(q) => Some(q),
                Err(s) => { println!("Error in line '{}': {}", m, s); None },
            })
        .collect();

    let mut stacks = Vec::with_capacity(10);
    for line in input
            .lines()
            .take_while(|line| !line.starts_with(" 1")) {
        let n = line.len() / CRATE_WIDTH + 1;
        for i in 0..n {
            let s: &str = &line[(i*CRATE_WIDTH)..];
            if s.starts_with("[") {
                while i >= stacks.len() {
                    stacks.push(VecDeque::with_capacity(10));
                }
                stacks[i].push_front(s.as_bytes()[1])
            }
        }
    }

    let p1 = simulate_with_cratemover_9000(&moves, &stacks);
    let p2 = simulate_with_cratemover_9001(&moves, &mut stacks);

    Solution::new(p1, p2)
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_crates = s[5..7].trim_end().parse::<usize>().map_err(|_| "Invalid number of crates value.")?;
        let src_offset = if num_crates > 9 { 13 } else { 12 };
        let dst_offset = src_offset + 5;
        let source = s[src_offset..(src_offset+1)].parse::<usize>().map_err(|_| "Invalid source value.")?;
        let dest = s[dst_offset..(dst_offset+1)].parse::<usize>().map_err(|_| "Invalid destination value.")?;
        Ok(Instruction { num_crates, source: source - 1, dest: dest - 1})
    }
}
