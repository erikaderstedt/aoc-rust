use crate::common::Solution;

fn answers_in_string(s: &str) -> u32 {
    s.chars().filter(|&c| c.is_ascii_lowercase())
        .fold(0u32, |a, c| a | (1u32 << ((c as u8) - ('a' as u8))))
}

pub fn solve(input: &str) -> Solution {
    let groups: Vec<&str> = input.split("\n\n").collect();

    let p1: u32 = groups.iter().map(|group| answers_in_string(group).count_ones()).sum();
    
    let p2: u32 = groups.iter()
        .map(|group| group.lines().fold((1<<26)-1, |a, s| a & answers_in_string(s)).count_ones())
        .sum();
     
    Solution::new(p1,p2)
}
