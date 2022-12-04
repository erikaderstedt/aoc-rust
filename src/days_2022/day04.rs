// https://adventofcode.com/2022/day/4

use crate::common::Solution;
use std::str::FromStr;

struct Elf {
    start: usize,
    stop: usize,
}

impl Elf {
    fn contains(&self, other: &Elf) -> bool {
        self.start <= other.start && self.stop >= other.stop
    }

    fn overlaps(&self, other: &Elf) -> bool {        
        self.stop >= other.start && other.start >= self.start
    }
}

pub fn solve(input: &str) -> Solution {

    let section_assignment_pairs: Vec<(Elf,Elf)> = input
        .lines()
        .flat_map(|line| {
            line
                .split_once(",")
                .map_or(None, |(e1, e2)|
                    match (e1.parse::<Elf>(), e2.parse::<Elf>()) {
                        (Ok(elf1), Ok(elf2)) => Some((elf1, elf2)),
                        _ => None,
                    }
                )
        })
        .collect();

    let p1 = section_assignment_pairs.iter().filter(|e| e.0.contains(&e.1) || e.1.contains(&e.0)).count();
    let p2 = section_assignment_pairs.iter().filter(|e| e.0.overlaps(&e.1) || e.1.overlaps(&e.0)).count();
    
    Solution::new(p1, p2)
}

impl FromStr for Elf {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('-') {
            Some((start, stop)) => {
                let start = start.parse::<usize>().map_err(|_| "Invalid start value.")?;
                let stop = stop.parse::<usize>().map_err(|_| "Invalid stop value.")?;
                Ok(Elf { start, stop })
            },
            _ => Err("Malformed line."),
        }
    }
}
