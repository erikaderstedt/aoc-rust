// https://adventofcode.com/2023/day/12

use std::str::FromStr;
use crate::common::{Solution, parsed_from_each_line};

#[derive(PartialEq, Eq, Clone)]
enum SpringState {
    Unknown,
    Damaged,
    Operational,
}

struct ConditionRecord {
    springs: Vec<SpringState>,
    damaged_sequences: Vec<usize>,
}

impl ConditionRecord {

    fn num_matches(&self) -> usize {
        let mut cache: Vec<Vec<Option<usize>>> = vec![vec![None; self.damaged_sequences.len()]; self.springs.len()];
        self.num_matches_internal(0, 0, &mut cache)
    }

    fn num_matches_internal(&self, spring: usize, damaged_sequence: usize, cache: &mut Vec<Vec<Option<usize>>>) -> usize {
        let mut arrangement_count = 0;

        // The next sequence can either start here, or not start here, both, or neither
        if self.springs[spring] != SpringState::Damaged && spring + 1 < self.springs.len() {
            // Try at a later point if that hasn't been done yet.
            arrangement_count += match cache[spring+1][damaged_sequence] {
                Some(existing) => existing,
                None => self.num_matches_internal(spring + 1, damaged_sequence, cache),
            };
        }

        let end = spring + self.damaged_sequences[damaged_sequence];
        if  end <= self.springs.len() &&
            self.springs[spring..end].iter().all(|x| *x != SpringState::Operational) &&
            (end == self.springs.len() || self.springs[end] != SpringState::Damaged) {
            // It fits here.
            if damaged_sequence + 1 == self.damaged_sequences.len() {
                // Are we at the last damaged sequence, but there are damaged springs left in the springs list?
                arrangement_count += if self.springs.iter().skip(end + 1).any(|x| *x == SpringState::Damaged) { 0 } else { 1 };
            } else if end + 1 < self.springs.len() {
                // Step ahead to next damaged sequence if that hasn't been done yet.                
                arrangement_count += match cache[end+1][damaged_sequence+1] {
                    Some(existing) => existing,
                    None => self.num_matches_internal(end + 1, damaged_sequence + 1, cache),
                }
            }
        }

        cache[spring][damaged_sequence] = Some(arrangement_count);
        arrangement_count
    }

    fn unfold(&self, times: usize) -> ConditionRecord {
        ConditionRecord { 
            springs: self.springs.iter()
                .chain(vec![&SpringState::Unknown])
                .cycle()
                .take(self.springs.len()*times + times - 1)
                .cloned()
                .collect(), 
            damaged_sequences: self.damaged_sequences.iter().cycle().take(self.damaged_sequences.len()*times).cloned().collect() }
    }
}

pub fn solve(input: &str) -> Solution {
    let recs: Vec<ConditionRecord> = parsed_from_each_line(input);

    let p1: usize = recs.iter().map(|r| r.num_matches()).sum();
    let p2: usize = recs.iter().map(|r| r.unfold(5).num_matches()).sum();

    Solution::new(p1, p2)
}

impl FromStr for ConditionRecord {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((s2, n)) = s.split_once(" ") {

            let springs = s2.chars().map(|c| match c {
                '?' => Ok(SpringState::Unknown),
                '.' => Ok(SpringState::Operational),
                '#' => Ok(SpringState::Damaged),
                _ => return Err("Invalid spring state."),
            }).collect();
            match springs {
                Ok(springs) => {
                    let damaged_sequences = n.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
                    Ok(ConditionRecord { springs, damaged_sequences })
                },
                Err(s) => Err(s),
            }            
        } else {
            Err("Malformed record")
        }
    }
}