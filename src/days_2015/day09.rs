// https://adventofcode.com/2015/day/9

use crate::common::Solution;
use std::collections::HashMap;
use itertools::Itertools;

pub fn solve(input: &str) -> Solution {
    let location_names: HashMap<&str, u16> = input
        .split_whitespace()
        .filter(|s| s.chars().next().unwrap().is_uppercase())
        .sorted()
        .dedup()
        .scan(0, |acc, s| {
            *acc += 1;
            Some((s, *acc - 1)) })
        .collect();
    
    let graph: HashMap<u16,HashMap<u16, usize>> = location_names
        .iter()
        .map(|(&name,code)| {
            (code.clone(), input.lines()
                .filter(|line| line.contains(name))
                .map(|line| {
                    let words: Vec<&str> = line.split(" ").collect();
                    if words[0] == name {
                        (location_names[words[2]], words[4].parse::<usize>().unwrap())
                    } else {
                        (location_names[words[0]], words[4].parse::<usize>().unwrap())
                    }
                })
                .collect())
        })
        .collect();
    
    let len = graph.len() as u16;
    // Up to 8! configurations. Around 40000. Get them all.
    let distances: Vec<usize> = (0..len)
        .permutations(graph.len())
        .map(|k| (0..(len-1)).map(|i| graph[&k[i as usize]][&k[i as usize +1]]).sum::<usize>())
        .collect();

    let p1 = distances.iter().min().unwrap();
    let p2 = distances.iter().max().unwrap();

    Solution::new(p1,p2)
}
