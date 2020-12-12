use crate::common::Solution;
use crate::common::parsed_from_each_line;
struct State { consecutive_ones: usize, branches: usize }

pub fn solve(input: &str) -> Solution {
    let mut nums:Vec<i64> = parsed_from_each_line(input);
    let device = nums.iter().max().unwrap() + 3;
    nums.push(0); 
    nums.sort();
    nums.push(device);
    let num_diffs = nums.len() - 1;    

    let diffs: Vec<i64> = nums.iter().take(num_diffs).zip(nums.iter().skip(1)).map(|(a,b)| b - a).collect();
    let p1 = diffs.iter().filter(|x| **x == 1).count() * diffs.iter().filter(|x| **x == 3).count();
    let p2 = diffs.into_iter().scan(State { consecutive_ones: 0, branches: 1 }, |state, diff| {
        match diff {
            1 => { state.consecutive_ones += 1 },
            3 => { state.branches *= match state.consecutive_ones {
                0..=1 => 1, 2 => 2, 3 => 4, 4 => 7,
                _ => panic!("This sequence of 1s was too long!"), };
                   state.consecutive_ones = 0; },
            _ => panic!("Unexpected diff found in sequence"),
        };
        Some(state.branches)
    }).last().unwrap();

    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}