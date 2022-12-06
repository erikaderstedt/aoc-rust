// https://adventofcode.com/2022/day/5

use crate::common::{Solution};
use std::{str::FromStr, collections::VecDeque, str};

const CRATE_WIDTH: usize = 4;

struct Instruction {
    num_crates: usize,
    source: usize,
    dest: usize
}

#[derive(Clone)]
struct CargoYard {
    stacks: Vec<VecDeque<u8>>, 
}

impl CargoYard {

    fn top(&self) -> String {
        let tops: Vec<u8> = self.stacks.iter().map(|s| s.back().unwrap().clone()).collect();
        str::from_utf8(&tops[..]).unwrap().to_string()
    }

    fn simulate_with_cratemover_9000(&mut self, moves: &Vec<Instruction>) -> String {    
        for m in moves.iter() {
            for _ in 0..m.num_crates {
                let top_crate = self.stacks[m.source].pop_back().unwrap();
                self.stacks[m.dest].push_back(top_crate);
            }
        }
        self.top()
    }

    fn simulate_with_cratemover_9001(&mut self, moves: &Vec<Instruction>) -> String {
        for m in moves.iter() {
            let num_crates_in_source = self.stacks[m.source].len();
            let mut p = self.stacks[m.source].split_off( num_crates_in_source - m.num_crates);
            self.stacks[m.dest].append(&mut p);
        }
    
        self.top()
    }
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
    let mut yard = input.parse::<CargoYard>().unwrap();

    Solution::new(yard.clone().simulate_with_cratemover_9000(&moves), 
            yard.simulate_with_cratemover_9001(&moves))
}

impl FromStr for CargoYard {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
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
        Ok(CargoYard { stacks })
    }
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
