// https://adventofcode.com/2025/day/10

use crate::common::{Solution, parsed_from_each_line};
use std::str::FromStr;



#[derive(Debug)]
struct Machine {
    target: u64,
    buttons: Vec<u64>,
    joltage_requirements: Vec<usize>,
}

impl Machine {
    
    fn minimum_required_button_presses(&self) -> usize {
        // No button will be pressed more than once.
        // This means that there are 2^N different states to enumerate
        // where N is the number of buttons.
        // 0 - not pressed, 1 - pressed
        // Just enumerate these, find which have the correct output
        // and minimize by popcount
        (1..(1<<self.buttons.len()))
            .filter_map(|i: usize| if self.buttons
                .iter()
                .enumerate()
                .filter(|(j, _)| ((1 << j) & i) > 0)
                .fold(0u64, |acc, (_, b)| acc ^ b) == self.target {
                    Some(i.count_ones() as usize)
                } else {
                    None
                })
            .min()
            .unwrap()
    }
}

pub fn solve(input: &str) -> Solution {
    let machines: Vec<Machine> = parsed_from_each_line(input);

    let p1 = machines.iter().map(|m| m.minimum_required_button_presses()).sum::<usize>();
    let p2 = 0;

    // I've been unable to solve part 2 :(
    // I really don't want to use z3. Last time I managed to avoid it by calculating the solution instead.
    // Tried that today but no luck so far.

    Solution::new(p1, p2)
}

impl FromStr for Machine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l: Vec<&str> = s.split(' ').collect();
        let target = l[0]
            .replace("[", "")
            .replace("]", "")
            .chars()
            .map(|u| if u == '#' { 1u64 } else { 0u64 } )
            .enumerate()
            .fold(0, |acc, (index, v)| acc + (v << (index as u64)));
        let joltage = l[l.len()-1]
            .replace("{", "")
            .replace("}", "")
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let buttons = l[1..(l.len()-1)].iter()
            .map(|s| s
                .replace("(", "")
                .replace(")", "")
                .split(',')
                .map(|q| q.parse::<u64>().unwrap())
                .fold(0, |acc, value| acc + (1 << value)))
            .collect();
        Ok( Machine { target, buttons, joltage_requirements: joltage })
    }
}
