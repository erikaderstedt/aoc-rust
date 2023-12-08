// https://adventofcode.com/2023/day/8

use std::collections::{HashMap, HashSet};
use std::str;

use crate::common::Solution;

pub fn solve(input: &str) -> Solution {

    let m: HashMap<&str, (&str,&str)> = input.lines().skip(2)
    .map(|line| (str::from_utf8(&line.as_bytes()[0..3]).unwrap(), 
    (str::from_utf8(&line.as_bytes()[7..10]).unwrap(), 
    str::from_utf8(&line.as_bytes()[12..15]).unwrap())))
    .collect();

    let dirs = input.lines().next().unwrap();

    let travel_to_zzz = |location: &str| -> usize {
        let mut p = 0;
        let mut l: &str = location;
        for d in dirs.chars().cycle() {
            match d {
                'L' => { l = m[l].0; },
                'R' => { l = m[l].1; },
                _ => panic!("asdfa")
            }
            p += 1;
            if l == "ZZZ" { break }
        }
        p
    };

    let travel_to_ending_in_z = |location: &str| -> usize {
        let mut p = 0;
        let mut l: &str = location;
        for d in dirs.chars().cycle() {
            match d {
                'L' => { l = m[l].0; },
                'R' => { l = m[l].1; },
                _ => panic!("asdfa")
            }
            p += 1;
            if l.ends_with("Z") { break }
        }
        p
    };

    let p1 = travel_to_zzz("AAA");

    let ghost_nodes: Vec<&str> = m.keys().cloned().filter(|s| s.ends_with("A")).collect();

    let p2_parts: HashSet<usize> = ghost_nodes.into_iter()
        .map(travel_to_ending_in_z)
        .map(|p| prime_factorization::Factorization::run(p as u32)
                .factors.into_iter().map(|d| d as usize))
        .flatten()
        .collect();
        
    let p2: usize = p2_parts.into_iter().product();

    Solution::new(p1,p2)
}
