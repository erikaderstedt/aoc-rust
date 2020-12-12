use crate::common::Solution;
use crate::common::parsed_from_each_line;

const PREAMBLE_LENGTH: usize = 25;

pub fn solve(input: &str) -> Solution {
    let nums: Vec<usize> = parsed_from_each_line(input);
    let p1: usize = nums.iter().skip(PREAMBLE_LENGTH)
        .enumerate()
        .find_map(|(idx, num)| {
            match !nums[idx..(idx + PREAMBLE_LENGTH)].iter()
                .any(|&p| nums[idx..(idx + PREAMBLE_LENGTH)].contains(&(num - p))) {
                true => Some(*num),
                false => None
            }})
        .unwrap();

    let p2 = (2..nums.len()).find_map(|window_length| {
        match nums.windows(window_length).find(|w| w.iter().sum::<usize>() == p1) {
            Some(w) => Some(w.iter().min().unwrap() + w.iter().max().unwrap()),
            None => None,
        }
    }).unwrap();
    
    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}